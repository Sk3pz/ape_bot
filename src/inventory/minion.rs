use serde::{Deserialize, Serialize};
use serenity::all::Timestamp;

const MINION_PRODUCTION_PER_HOUR: u32 = 20;
pub const MINION_BASE_MAX_SLUDGE: u32 = 600;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

    pub fn minute_sludge_production(&self) -> u32 {
        MINION_PRODUCTION_PER_HOUR * self.level as u32
    }

    pub fn get_sludge_produced(&self) -> u32 {
        let now = Timestamp::now();

        let duration = now.signed_duration_since(self.mining_start.fixed_offset());

        let seconds = duration.num_seconds() as f64;

        let hours_passed = seconds / 3600.0;

        let produced = (self.minute_sludge_production() as f64 * hours_passed) as u32;

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