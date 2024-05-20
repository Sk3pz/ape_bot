use serde::{Deserialize, Serialize};
use serenity::all::Timestamp;

const MINION_SLUDE_HOURLY_PRODUCTION: u32 = 1;
const MINION_BASE_MAX_SLUDGE: u32 = 100;

#[derive(Serialize, Deserialize, Clone)]
pub struct Minion {
    pub level: u8,
    pub mining_start: Timestamp,
}

impl Minion {
    pub fn new() -> Self {
        Self {
            level: 1,
            mining_start: Timestamp::now(),
        }
    }

    pub fn hourly_sludge_production(&self) -> u32 {
        MINION_SLUDE_HOURLY_PRODUCTION * self.level as u32
    }

    pub fn get_sludge_produced(&self) -> u32 {
        let now = Timestamp::now();

        let duration = now.signed_duration_since(self.mining_start.fixed_offset());

        let hours = duration.num_hours();

        let produced = self.hourly_sludge_production() * hours as u32;

        if produced > MINION_BASE_MAX_SLUDGE {
            MINION_BASE_MAX_SLUDGE
        } else {
            produced
        }
    }

    pub fn is_full(&self) -> bool {
        self.get_sludge_produced() >= MINION_BASE_MAX_SLUDGE
    }

    pub fn mine(&mut self) {
        self.mining_start = Timestamp::now();
    }
}