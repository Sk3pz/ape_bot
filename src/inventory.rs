pub mod minion;
pub mod item;
pub mod super_drill;

use serde::{Deserialize, Serialize};
use crate::inventory::super_drill::SuperDrill;

pub const MAX_INVENTORY_SIZE: u8 = 16;

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<item::InventoryItem>,
}

impl Inventory {

    pub fn get_minions(&self) -> Vec<minion::Minion> {
        let mut minions = Vec::new();
        for item in self.items.iter() {
            if let item::InventoryItem::Minion(minion) = item {
                minions.push(minion.clone());
            }
        }

        minions
    }

    pub fn get_super_drill(&self) -> Option<SuperDrill> {
        for item in self.items.iter() {
            if let item::InventoryItem::SuperDrill(super_drill) = item {
                return Some(super_drill.clone());
            }
        }

        None
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= MAX_INVENTORY_SIZE as usize
    }
}

impl Clone for Inventory {
    fn clone(&self) -> Self {
        let mut items = Vec::new();
        for item in self.items.iter() {
            items.push(item.clone());
        }

        Self {
            items
        }
    }
}