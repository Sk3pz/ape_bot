use rand::{Rng, thread_rng};
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, Message, Timestamp, UserId};
use crate::userfile::UserValues;

pub struct SludgeMonsterBattle {
    pub boss_health: u32,
    pub player_health: u32,
    pub thumbnail: String,
    pub initial_health: u32
}

impl SludgeMonsterBattle {

    pub fn new() -> Self {
        // create a new sludge monster battle
        let boss_health = thread_rng().gen_range(1..=5) * 100;
        Self {
            boss_health,
            player_health: 100,
            thumbnail: if boss_health > 400 {
                "large_sludge.jpeg".to_string()
            } else if boss_health > 200 {
                "medium_sludge.jpeg".to_string()
            } else {
                "small_sludge.jpeg".to_string()
            },
            initial_health: boss_health
        }
    }

    pub fn attack(&mut self) -> u32 {
        // calculate damage between 0 and 25 hp
        let damage = thread_rng().gen_range(0..=25);
        if damage > self.boss_health {
            self.boss_health = 0;
            return damage;
        }
        self.boss_health -= damage;
        damage
    }

    pub fn attempt_flee(&mut self) -> bool {
        // calculate if the user flees based on health
        thread_rng().gen_range(0..self.player_health) == 0
    }

    pub fn boss_turn(&mut self) -> u32 {
        // calculate damage between 0 and 25 hp
        let damage = thread_rng().gen_range(0..=10);
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

    pub fn craft_embed(&self, message: String) -> CreateEmbed {
        // create an embed message for the battle
        CreateEmbed::new()
            .title("Sludge Monster Battle")
            .description(message)
            .color(Colour::DARK_GREEN)
            .fields(
                vec![
                    ("Boss Health", format!("{}", self.boss_health), true),
                    ("Your Health", self.player_health.to_string(), true),
                    ("Options: ", "`attack` `run` or `surrender`".to_string(), false),
                ]
            )
    }

    fn handle_player_death(&self, user: UserId) -> (CreateEmbed, bool) {
        let cost  = thread_rng().gen_range(1..=20) * 100;

        let mut user_file = UserValues::get(&user);

        let balance = user_file.get_bananas();
        if balance < cost {
            user_file.remove_bananas(balance);
        } else {
            user_file.remove_bananas(cost);
        }

        return (CreateEmbed::new()
                    .title("Defeat!")
                    .description("The sludge monster defeated you and has stolen some bananas!")
                    .field("Lost Bananas:", format!("{}:banana:", cost), false)
                    .color(Colour::RED)
                    .timestamp(Timestamp::now())
                    .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                true);
    }

    pub fn handle_message(&mut self, msg: &Message) -> (CreateEmbed, bool) {
        let content = msg.content.as_str().to_lowercase();

        match content.trim() {
            "attack" => {
                let attack = self.attack();

                /*
                    SLUDGE Monster REWARDS:
                    500 HP: 20,000 - 100,000
                    400 HP: 15,000 - 75,000
                    300 HP: 10,000 - 50,000
                    200 HP: 5,000 - 25,000
                    100 HP: 2,000 - 10,000
                */

                if self.boss_health == 0 {
                    let reward_high = if self.initial_health == 500 {
                        100
                    } else if self.initial_health == 400 {
                        75
                    } else if self.initial_health == 300 {
                        50
                    } else if self.initial_health == 200 {
                        25
                    } else {
                        10
                    };
                    let reward_low = reward_high / 5;
                    let reward = thread_rng().gen_range(reward_low..=reward_high) * 1000;

                    let mut user_file = UserValues::get(&msg.author.id);
                    user_file.add_bananas(reward);

                    return (CreateEmbed::new()
                                .title("Sludge Monster Defeated!")
                                .description("You have defeated the Sludge Monster!")
                                .field("Reward:", format!("{}:banana:", reward), false)
                                .color(Colour::DARK_GREEN)
                                .timestamp(Timestamp::now())
                                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                            true);
                }

                let boss_attack = self.boss_turn();

                if self.player_health == 0 {
                    return self.handle_player_death(msg.author.id);
                }

                (self.craft_embed(format!("You attacked the boss for {} damage, but it attacked you for {}!", attack, boss_attack)), false)
            }
            "run" => {
                let flee = self.attempt_flee();

                if flee {
                    return (CreateEmbed::new()
                                .title("Flee!")
                                .description("You have successfully fled from the sludge monster.")
                                .color(Colour::DARK_GREEN)
                                .timestamp(Timestamp::now())
                                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©")),
                            true);
                }

                let boss_attack = self.boss_turn();

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