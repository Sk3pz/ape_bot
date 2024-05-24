use rand::{Rng, thread_rng};
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, Message, Timestamp, UserId};
use crate::inventory::item::InventoryItem;
use crate::mine_data::Enemy;
use crate::userfile::UserValues;

pub struct MineBattle {
    pub enemy: Enemy,
    pub sludge_value: u32,
    pub enemy_health: u32,
    pub player_health: u32,
    pub initial_health: u32,
    pub has_prayed: bool,
}

impl MineBattle {

    pub fn new(enemy: Enemy, sludge_value: u32) -> Self {
        // create a new mine battle
        let enemy_health = thread_rng().gen_range(enemy.health.clone());
        // convert to round 100 base number
        let enemy_health = enemy_health - (enemy_health % 100);
        Self {
            enemy,
            sludge_value,
            enemy_health,
            player_health: 100,
            initial_health: enemy_health,
            has_prayed: false,
        }
    }

    pub fn attack(&mut self) -> u32 {
        // calculate damage between 0 and 25 hp
        let damage = thread_rng().gen_range(self.enemy.damage.clone());
        if damage > self.enemy_health {
            self.enemy_health = 0;
            return damage;
        }
        self.enemy_health -= damage;
        damage
    }

    pub fn attempt_flee(&mut self) -> bool {
        // calculate if the user flees based on health
        thread_rng().gen_range(0..self.player_health) == 0
    }

    pub fn enemy_turn(&mut self) -> u32 {
        // calculate damage between 0 and 25 hp
        let damage = thread_rng().gen_range(self.enemy.damage.clone());
        if damage > self.player_health {
            self.player_health = 0;
            return damage;
        }
        self.player_health -= damage;
        damage
    }

    pub fn heal_player(&mut self, amt: u32) {
        // heal the player for 25 hp
        self.player_health += amt;
        if self.player_health > 100 {
            self.player_health = 100;
        }
    }

    pub fn use_item(&mut self, item: InventoryItem, user: UserId) -> (bool, (CreateEmbed, bool)) {
        // use an item in the battle
        match item {
            InventoryItem::HealingPotion { health } => {
                self.heal_player(health);
                (true, (self.craft_embed(format!("You healed for {}hp!", health)), false))
            }
            InventoryItem::SpellTome { name, damage } => {
                let damage = thread_rng().gen_range(damage);
                if damage > self.enemy_health {
                    self.enemy_health = 0;
                    return (true, self.handle_win(user));
                }
                self.enemy_health -= damage;
                (true, (self.craft_embed(format!("You used {} and dealt {} damage!", name, damage)), false))
            }
            _ => { (false, (self.craft_embed("You can not use that item here!".to_string()), false)) }
        }
    }

    pub fn craft_embed(&self, message: String) -> CreateEmbed {
        // create an embed message for the battle
        CreateEmbed::new()
            .title(format!("{} Battle", self.enemy.name))
            .description(message)
            .color(Colour::DARK_GREEN)
            .fields(
                vec![
                    ("Creature Health", format!("{}", self.enemy_health), true),
                    ("Your Health", self.player_health.to_string(), true),
                    ("Options: ", "`attack`, `run`, `item {inventory slot #}` or `surrender`".to_string(), false),
                ]
            )
    }

    pub fn handle_win(&self, user: UserId) -> (CreateEmbed, bool) {
        let mut user_file = UserValues::get(&user);

        let reward_chance = thread_rng().gen_range(0..3);
        match reward_chance {
            0 => { // sludge mined
                let reward = if self.enemy.reward_scaling {
                    // multiply the winnings by health / 100
                    self.sludge_value * self.initial_health / 100
                } else {
                    self.sludge_value
                };
                user_file.add_bananas(reward as u64);
                (CreateEmbed::new()
                     .title("Victory!")
                     .description(format!("You have defeated the {} and have been rewarded with some bananas!", self.enemy.name))
                     .field("Reward:", format!("{}:banana:", reward), false)
                     .color(Colour::DARK_GREEN)
                     .timestamp(Timestamp::now())
                     .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                 true)
            }
            1 => { // item found
                let item = self.enemy.drops.random_item();
                if user_file.file.inventory.is_full() {
                    return (CreateEmbed::new()
                                .title("Victory!")
                                .description(format!("You have defeated the {} and have been rewarded with an item, but your inventory is full!", self.enemy.name))
                                .field("Reward:", format!(":x: {}", item), false)
                                .color(Colour::RED)
                                .timestamp(Timestamp::now())
                                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                            true);
                }
                user_file.add_item(item.clone());
                (CreateEmbed::new()
                     .title("Victory!")
                     .description(format!("You have defeated the {} and have been rewarded with an item!", self.enemy.name))
                     .field("Reward:", format!("{}", item), false)
                     .color(Colour::DARK_GREEN)
                     .timestamp(Timestamp::now())
                     .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                 true)
            }
            2 => { // super nanners
                let nanners = thread_rng().gen_range(1..=5);
                user_file.add_super_nanners(nanners);
                (CreateEmbed::new()
                     .title("Victory!")
                     .description(format!("You have defeated the {} and have been rewarded with some super nanners!", self.enemy.name))
                     .field("Reward:", format!("{}:zap:", nanners), false)
                     .color(Colour::DARK_GREEN)
                     .timestamp(Timestamp::now())
                     .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                 true)
            }
            _ => unreachable!()
        }
    }

    pub fn handle_player_death(&self, user: UserId) -> (CreateEmbed, bool) {
        let mut user_file = UserValues::get(&user);
        let nanners = user_file.get_bananas();
        let cost_min = nanners / 5;
        let cost_max = nanners / 2;

        let cost  = thread_rng().gen_range(cost_min..=cost_max);

        let balance = user_file.get_bananas();
        if balance < cost {
            user_file.remove_bananas(balance);
        } else {
            user_file.remove_bananas(cost);
        }

        return (CreateEmbed::new()
                    .title("Defeat!")
                    .description(format!("The {} defeated you and has stolen some bananas!", self.enemy.name))
                    .field("Lost Bananas:", format!("{}:banana:", cost), false)
                    .color(Colour::RED)
                    .timestamp(Timestamp::now())
                    .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                true);
    }

    pub fn handle_message(&mut self, msg: &Message) -> (CreateEmbed, bool) {
        let content = msg.content.as_str().to_lowercase();
        let mut split = content.split_whitespace();
        let first = split.next().unwrap();

        match first {
            "attack" => {
                let attack = self.attack();

                if self.enemy_health == 0 {
                    return self.handle_win(msg.author.id);
                }

                let boss_attack = self.enemy_turn();

                if self.player_health == 0 {
                    return self.handle_player_death(msg.author.id);
                }

                (self.craft_embed(format!("You attacked the {} for {} damage, and it attacked you for {} damage!",
                                          self.enemy.name, attack, boss_attack)), false)
            }
            "item" => {
                // get the item the user is trying to use as a u8
                let Ok(slot) = split.next().unwrap().parse::<u8>() else {
                    return (self.craft_embed("**INVALID ITEM** To use an item, type `item #`, where # is the slot in your inventory (see `/inventory`)"
                        .to_string()), false)
                };

                let mut user_file = UserValues::get(&msg.author.id);

                let items = user_file.get_items();

                // get the item from the user's inventory
                let Some(item) = items.get(slot as usize) else {
                    return (self.craft_embed("**INVALID ITEM** To use an item, type `item #`, where # is the slot in your inventory (see `/inventory`)"
                        .to_string()), false)
                };

                let (used, data) = self.use_item(item.clone(), msg.author.id);

                // remove the item
                if used {
                    user_file.remove_item_index(slot as usize);
                }

                data
            }
            "pray" => {
                if self.has_prayed {
                    return (self.craft_embed("You have already prayed this battle! (This easter egg can't be too powerful!)".to_string()), false)
                }

                let heal = thread_rng().gen_range(1..=25);
                self.heal_player(heal);
                self.has_prayed = true;
                (self.craft_embed(format!("You prayed and healed for {}hp!", heal)), false)
            }
            "run" => {
                let flee = self.attempt_flee();

                if flee {
                    return (CreateEmbed::new()
                                .title("Flee!")
                                .description("You have successfully fled from the creature.")
                                .color(Colour::DARK_GREEN)
                                .timestamp(Timestamp::now())
                                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                            true);
                }

                let boss_attack = self.enemy_turn();

                if self.player_health == 0 {
                    return self.handle_player_death(msg.author.id);
                }

                (self.craft_embed(format!("You failed to flee, and the sludge monster attacked you for {} damage!", boss_attack)), false)
            }
            "surrender" => {
                return self.handle_player_death(msg.author.id);
            }
            _ => {
                (self.craft_embed("Me no understand!".to_string()), false)
            }
        }
    }

}