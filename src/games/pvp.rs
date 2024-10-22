use std::ops::RangeInclusive;
use rand::Rng;
use serenity::all::{Colour, Context, CreateEmbed, Message, UserId};
use serenity::builder::CreateEmbedFooter;
use crate::inventory::item::InventoryItem;
use crate::userfile::UserValues;

pub enum PvPModFlag {
    NoItem,
    MaxPlayers(u64),
    CustomStartHealth(u32),
    CustomMaxHealth(u32),
    DamageRange(u32, u32),
}

struct Player {
    user: UserId,
    health: u32,
}

pub struct PvPArena {
    pub(crate) stake: u64,
    host: UserId,
    players: Vec<Player>,
    turn: u16,
    started: bool,

    // modifiers
    max_players: u64,
    max_health: u32,
    base_health: u32,
    damage_range: RangeInclusive<u32>,
    enable_items: bool,

    total_players: u64,
}

impl PvPArena {

    pub fn new(host: UserId, stake: u64, flags: Vec<PvPModFlag>) -> Self {
        let mut enable_items = true;
        let mut max_players = 2;
        let mut max_health = 100;
        let mut base_health = 100;
        let mut damage_range: RangeInclusive<u32> = 0..=10;

        for flag in flags {
            match flag {
                PvPModFlag::NoItem => enable_items = false,
                PvPModFlag::MaxPlayers(players) => max_players = players,
                PvPModFlag::CustomStartHealth(health) => base_health = health,
                PvPModFlag::CustomMaxHealth(health) => max_health = health,
                PvPModFlag::DamageRange(min, max) => damage_range = min..=max,
            }
        }

        Self {
            stake,
            host: host.clone(),
            players: vec![Player { user: host, health: 100 }],
            turn: 0,
            started: false,

            max_players,
            max_health,
            base_health,
            damage_range,
            enable_items,

            total_players: 1,
        }
    }

    pub fn add_player(&mut self, user: UserId) {
        self.players.push(Player {
            user,
            health: self.base_health as u32,
        });

        self.total_players += 1;
    }

    pub fn remove_player(&mut self, user: UserId) {
        let index = self.players.iter().position(|p| p.user == user).unwrap();
        self.players.remove(index);
    }

    pub fn is_host(&self, user: UserId) -> bool {
        self.host == user
    }

    pub fn is_running(&self) -> bool {
        self.started
    }

    pub fn can_join(&self) -> bool {
        self.players.len() < self.max_players as usize && !self.is_running()
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
        if self.turn >= self.players.len() as u16 {
            self.turn = 0;
        }
    }

    // todo: make list show health for all players
    pub fn list_players(&self, ctx: &Context) -> Vec<String> {
        // get the player names
        let mut names = Vec::new();
        for player in &self.players {
            let user = player.user.to_user_cached(&ctx.cache);
            let name = if let Some(usr) = user {
                usr.global_name.clone().unwrap()
            } else {
                "unknown".to_string()
            };
            names.push(name);
        }

        names
    }

    // todo: add leave option for all players here
    pub async fn handle_prestart_message(&mut self, ctx: &Context, msg: &Message) -> Option<(CreateEmbed, bool, Option<UserId>)> {
        let content = msg.content.as_str().to_lowercase();
        let mut split = content.split_whitespace();
        let first = split.next().unwrap();

        match first {
            // start the game (more than 1 player required)
            "start" => {
                // ensure the player is the host
                if !self.is_host(msg.author.id) {
                    return None;
                }

                // ensure there are at least 2 players
                if self.players.len() < 2 {
                    return Some((CreateEmbed::default()
                        .title("Not enough players to start the game.")
                        .description("You need at least 2 players to start the game."),
                        false, None));
                }

                // start the game
                self.started = true;

                // get the current player's name based on turn TODO : BROKEN
                let user = self.players[self.turn as usize].user.to_user(&ctx.http).await.unwrap().clone().global_name.unwrap_or("unknown".to_string());

                Some((CreateEmbed::default()
                          .title("The arena has started!")
                          .description(format!("It is {}'s turn to perform an action.\n  Health: {}", user, self.players[self.turn as usize].health))
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .color(Colour::RED)
                          .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                      false, None))
            }
            // end the game
            "end" => {
                // ensure the player is the host
                if !self.is_host(msg.author.id) {
                    return None;
                }

                // add back stake to users
                for p in &self.players {
                    let mut userfile = UserValues::get(&p.user);
                    userfile.add_bananas(self.stake);
                }

                Some((CreateEmbed::default()
                          .title("The arena has been closed.")
                          .description("All bananas have been refunded.")
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .color(Colour::RED),
                      true, None))
            }
            // list all players in the game
            "list" => {
                // get the player names
                let names = self.list_players(ctx);

                Some((CreateEmbed::default()
                          .title("Players in your arena")
                          .color(Colour::RED)
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .description(names.join("\n")),
                    false, None))
            }
            "leave" => {
                todo!()
            }
            // kick a player from the game
            "kick" => {
                // ensure the player is the host
                if !self.is_host(msg.author.id) {
                    return None;
                }

                // todo: ensure bananas get refunded to kicked user

                self.total_players -= 1;
                None
            }
            _ => { None }
        }
    }

    fn handle_win(&mut self, ctx: &Context, winner_id: UserId) -> (CreateEmbed, bool, Option<UserId>) {
        let winner = winner_id.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());

        let pot = self.stake * self.total_players;

        // add the pot to the user
        let mut userfile = UserValues::get(&winner_id);
        userfile.add_bananas(pot);

        (CreateEmbed::default()
                         .title(format!("{} has won the arena!", winner.clone()))
                         .color(Colour::RED)
                         .thumbnail("attachment://battle_monkey.jpeg")
                         .description(format!("{} has won the arena and gets {}:banana:!", winner, pot)),
                     true, None)
    }

    pub fn handle_message(&mut self, ctx: &Context, user: UserId, msg: &Message) -> Option<(CreateEmbed, bool, Option<UserId>)> {
        let content = msg.content.as_str().to_lowercase();
        let mut split = content.split_whitespace();
        let first = split.next().unwrap();

        // determine if it is the user's turn
        let is_turn = self.players[self.turn as usize].user == user;

        let last_2 = self.players.len() < 3;

        if is_turn {
            self.next_turn();
        }

        let next_turn_name = self.players[self.turn as usize].user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()).clone();

        let end = match first {
            "attack" => {
                // ensure it is the user's turn
                if !is_turn {
                    return Some((CreateEmbed::default()
                        .title("It is not your turn.")
                        .color(Colour::RED)
                        .thumbnail("attachment://battle_monkey.jpeg")
                        .description("Wait for your turn to attack."),
                        false, None));
                }

                // if player count is more than 2, user must specify which player to attack
                let target = if self.players.len() > 2 {
                    // check if the msg contains a mention
                    if msg.mentions.len() == 0 {
                        if self.turn > 0 {
                            self.turn -= 1;
                        } else {
                            self.turn = (self.players.len() - 1) as u16;
                        }
                        return Some((CreateEmbed::default()
                            .title("You must specify a player to attack.")
                            .color(Colour::RED)
                            .thumbnail("attachment://battle_monkey.jpeg")
                            .description("You must mention a player to attack.")
                            .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                            false, None));
                    }
                    let target = msg.mentions[0].id;

                    // ensure the target is in the game
                    if !self.players.iter().any(|p| p.user == target) {
                        if self.turn > 0 {
                            self.turn -= 1;
                        } else {
                            self.turn = (self.players.len() - 1) as u16;
                        }
                        return Some((CreateEmbed::default()
                            .title("Player not found.")
                            .color(Colour::RED)
                            .thumbnail("attachment://battle_monkey.jpeg")
                            .description("The player you are trying to attack is not in the game.")
                            .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                            false, None));
                    }
                    target
                } else {
                    // get the other player
                    self.players.iter().find(|p| p.user != user).unwrap().user
                };

                let target = self.players.iter_mut().find(|p| p.user == target).unwrap();

                // get the user's equipped item if they have one
                let mut userfile = UserValues::get(&user);

                if self.enable_items {
                    let item = userfile.get_equiped();
                    if let Some(InventoryItem::Weapon { name, damage, .. }) = item {

                        // generate the damage
                        let damage = rand::thread_rng().gen_range(damage);

                        // apply the damage to the target and return
                        if damage > target.health {
                            target.health = 0;
                        } else {
                            target.health -= damage;
                        }

                        let target_name = target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());

                        // todo: check if target is dead and handle if required
                        if target.health == 0 {
                            return if last_2 {
                                Some(self.handle_win(ctx, user))
                            } else {
                                Some((CreateEmbed::default()
                                          .title(format!("{} has defeated {} using {}!",
                                                         user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                         target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()), name))
                                          .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, self.players[self.turn as usize].health))
                                          .thumbnail("attachment://battle_monkey.jpeg")
                                          .color(Colour::RED)
                                          .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                      false, None))
                            }
                        }

                        let next_health = self.players[self.turn as usize].health;

                        return Some((CreateEmbed::default()
                                         .title(format!("{} has attacked {} for {} damage using {}!", user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                        target_name, damage, name))
                                         .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, next_health))
                                         .thumbnail("attachment://battle_monkey.jpeg")
                                         .color(Colour::RED)
                                         .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                     false, None));
                    }
                }

                // generate the damage
                let damage: u32 = rand::thread_rng().gen_range(self.damage_range.clone());

                // apply the damage to the target and return
                if damage > target.health {
                    target.health = 0;
                } else {
                    target.health -= damage;
                }

                if target.health == 0 {
                    return if last_2 {
                        Some(self.handle_win(ctx, user))
                    } else {
                        let target_name = target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());
                        let target = target.user;


                        Some((CreateEmbed::default()
                                  .title(format!("{} has defeated {}!",
                                                 user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                 target_name))
                                  .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, self.players[self.turn as usize].health))
                                  .thumbnail("attachment://battle_monkey.jpeg")
                                  .color(Colour::RED)
                                  .footer(CreateEmbedFooter::new("You can type `attack <@player>`, `item #`, `list` or `surrender`")),
                              false, Some(target)))
                    }
                }

                
                let target_name = target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());

                Some((CreateEmbed::default()
                                 .title(format!("{} has attacked {} for {} damage!", user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                target_name, damage))
                          .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, self.players[self.turn as usize].health))
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .color(Colour::RED)
                          .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                             false, None))
            }
            "item" => { // use an item
                // users can use healing potions on their turns

                // if items are disabled, return
                if !self.enable_items {
                    if is_turn {
                        if self.turn > 0 {
                            self.turn -= 1;
                        } else {
                            self.turn = (self.players.len() - 1) as u16;
                        }
                    }
                    return Some((CreateEmbed::default()
                                     .title("You can't do that here!")
                                     .color(Colour::RED)
                                     .thumbnail("attachment://battle_monkey.jpeg")
                                     .description("Items are disabled in this arena.")
                                     .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                 false, None));
                }

                // get the item number
                let Ok(slot) = split.next().unwrap().parse::<u8>() else {
                    if is_turn {
                        if self.turn > 0 {
                            self.turn -= 1;
                        } else {
                            self.turn = (self.players.len() - 1) as u16;
                        }
                    }
                    return Some((CreateEmbed::new()
                                     .title("Invalid Item!")
                                     .description("You must specify and item slot `item <slot #>`."),
                                 false, None))
                };

                // get the item
                let mut user_file = UserValues::get(&msg.author.id);
                let items = user_file.get_items();

                // 1 index slot
                let slot = slot - 1;
                
                let Some(item) = items.get(slot as usize) else {
                    if is_turn {
                        if self.turn > 0 {
                            self.turn -= 1;
                        } else {
                            self.turn = (self.players.len() - 1) as u16;
                        }
                    }
                    return Some((CreateEmbed::default()
                                     .title("Invalid Item!")
                                     .color(Colour::RED)
                                     .thumbnail("attachment://battle_monkey.jpeg")
                                     .description("You must specify a valid item slot.")
                                     .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                 false, None))
                };

                match item {
                    InventoryItem::HealingPotion { health } => {
                        // apply the healing to the user

                        // it will not count as their turn
                        if is_turn {
                            if self.turn > 0 {
                                self.turn -= 1;
                            } else {
                                self.turn = (self.players.len() - 1) as u16;
                            }
                        }

                        // get the user from players (turn is already incremented)
                        let current_player_loc = self.players.iter().position(|p| p.user == user).unwrap_or(0);
                        let current_player = &mut self.players[current_player_loc];
                        current_player.health += health;
                        if current_player.health > self.max_health {
                            current_player.health = self.max_health;
                        }
                        let current_player_name = current_player.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());

                        // remove the item from the user's inventory
                        user_file.remove_item_index(slot as usize);

                        if is_turn {
                            Some((CreateEmbed::default()
                                      .title(format!("{} has healed to {}hp!", current_player_name, health))
                                      .color(Colour::RED)
                                      .thumbnail("attachment://battle_monkey.jpeg")
                                      .description(format!("It is still your turn to perform an action.\n  Health: {}", current_player.health))
                                      .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                  false, None))
                        } else {
                            let next_user = &self.players[(self.turn) as usize];
                            let next_turn_name = next_user.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());
                            let name = user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string());

                            Some((CreateEmbed::default()
                                      .title(format!("{} has healed to {}hp!", name, health))
                                      .color(Colour::RED)
                                      .thumbnail("attachment://battle_monkey.jpeg")
                                      .description(format!("It is still {}'s to perform an action.\n  Health: {}", next_turn_name, next_user.health))
                                      .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                  false, None))
                        }
                    }
                    InventoryItem::SpellTome { name, damage } => {
                        // users can not use spell tomes on their turns
                        if !is_turn {
                            return Some((CreateEmbed::default()
                                             .title("It is not your turn.")
                                             .color(Colour::RED)
                                             .thumbnail("attachment://battle_monkey.jpeg")
                                             .description("Wait for your turn to attack."),
                                         false, None));
                        }

                        // if player count is more than 2, user must specify which player to attack
                        let target = if self.players.len() > 2 {
                            // check if the msg contains a mention
                            if msg.mentions.len() == 0 {
                                if self.turn > 0 {
                                    self.turn -= 1;
                                } else {
                                    self.turn = (self.players.len() - 1) as u16;
                                }
                                return Some((CreateEmbed::default()
                                                 .title("You must specify a player to attack.")
                                                 .color(Colour::RED)
                                                 .thumbnail("attachment://battle_monkey.jpeg")
                                                 .description("You must mention a player to attack. `item # @player`")
                                                 .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                             false, None));
                            }
                            let target = msg.mentions[0].id;

                            // ensure the target is in the game
                            if !self.players.iter().any(|p| p.user == target) {
                                if self.turn > 0 {
                                    self.turn -= 1;
                                } else {
                                    self.turn = (self.players.len() - 1) as u16;
                                }
                                return Some((CreateEmbed::default()
                                                 .title("Player not found.")
                                                 .color(Colour::RED)
                                                 .thumbnail("attachment://battle_monkey.jpeg")
                                                 .description("The player you are trying to attack is not in the game.")
                                                 .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                             false, None));
                            }
                            target
                        } else {
                            // get the other player
                            self.players.iter().find(|p| p.user != user).unwrap().user
                        };

                        let target = self.players.iter_mut().find(|p| p.user == target).unwrap();

                        // apply the damage to the target and return

                        let damage = rand::thread_rng().gen_range(damage.clone());

                        if damage > target.health {
                            target.health = 0;
                        } else {
                            target.health -= damage;
                        }

                        // remove the item from the user's inventory
                        user_file.remove_item_index(slot as usize);

                        if target.health == 0 {
                            return if last_2 {
                                Some(self.handle_win(ctx, user))
                            } else {
                                Some((CreateEmbed::default()
                                          .title(format!("{} has defeated {} using {} Tome!",
                                                         user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                         target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()), name))
                                          .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, self.players[self.turn as usize].health))
                                          .thumbnail("attachment://battle_monkey.jpeg")
                                          .color(Colour::RED)
                                          .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                      false, None))
                            }
                        }

                        Some((CreateEmbed::default()
                                     .title(format!("{} has attacked {} for {} damage using {} Tome!", user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()),
                                                    target.user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string()), damage, name))
                                     .description(format!("It is {}'s turn to perform an action.\n  Health: {}", next_turn_name, self.players[self.turn as usize].health))
                                     .thumbnail("attachment://battle_monkey.jpeg")
                                     .color(Colour::RED)
                                     .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                 false, None))
                    }
                    _ => {
                        if is_turn {
                            if self.turn > 0 {
                                self.turn -= 1;
                            } else {
                                self.turn = (self.players.len() - 1) as u16;
                            }
                        }
                        Some((CreateEmbed::default()
                                         .title("Invalid Item!")
                                         .color(Colour::RED)
                                         .thumbnail("attachment://battle_monkey.jpeg")
                                         .description("You cannot use that item here!")
                                         .footer(CreateEmbedFooter::new("You can type `attack @player`, `item #`, `list` or `surrender`")),
                                     false, None))
                    }
                }
            }
            "list" => {
                // get the player names
                let names = self.list_players(ctx);

                if is_turn {
                    if self.turn > 0 {
                        self.turn -= 1;
                    } else {
                        self.turn = (self.players.len() - 1) as u16;
                    }
                }

                Some((CreateEmbed::default()
                          .title("Players in your arena")
                          .color(Colour::RED)
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .description(names.join("\n")),
                      false, None))
            }
            "surrender" => {
                // give all other users a portion of the stake
                let stake = self.stake / self.players.len() as u64;
                for player in &self.players {
                    if player.user != user {
                        let mut userfile = UserValues::get(&player.user);
                        userfile.add_bananas(stake);
                    }
                }

                // check if there is less than 2 players and determine win
                if self.players.len() < 3 {
                    let index = self.players.iter().position(|p| p.user != user).unwrap();
                    return Some(self.handle_win(ctx, self.players[index].user.clone()));
                }

                Some((CreateEmbed::default()
                          .title(format!("{} surrendered!", user.to_user_cached(&ctx.cache).unwrap().clone().global_name.unwrap_or("unknown".to_string())))
                          .color(Colour::RED)
                          .thumbnail("attachment://battle_monkey.jpeg")
                          .description("You have forfeited the game and have been removed from the arena.")
                          .footer(CreateEmbedFooter::new("You can type `attack`, `item`, or `surrender`")),
                      false, Some(user)))
            }
            _ => {
                if is_turn {
                    if self.turn > 0 {
                        self.turn -= 1;
                    } else {
                        self.turn = (self.players.len() - 1) as u16;
                    }
                }
                None
            }
        };

        end
    }
}