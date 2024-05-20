pub mod minion;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub minions: Vec<minion::Minion>,
    pub super_drill: bool,
}

impl Clone for Inventory {
    fn clone(&self) -> Self {
        let mut minions = Vec::new();
        for minion in self.minions.iter() {
            minions.push((*minion).clone());
        }

        Self {
            minions,
            super_drill: self.super_drill,
        }
    }
}