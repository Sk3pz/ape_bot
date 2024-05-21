pub mod minion;
pub mod item;
mod super_drill;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<item::InventoryItem>,
}

impl Inventory {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_minions(&self) -> Vec<minion::Minion> {
        let mut minions = Vec::new();
        for item in self.items.iter() {
            if let item::InventoryItem::Minion(minion) = item {
                minions.push(minion.clone());
            }
        }

        minions
    }

    pub fn get_super_drill(&self) -> Option<super_drill> {
        for item in self.items.iter() {
            if let item::InventoryItem::SuperDrill(super_drill) = item {
                return Some(super_drill.clone());
            }
        }

        None
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