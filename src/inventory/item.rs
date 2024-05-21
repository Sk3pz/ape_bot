use serde::{Deserialize, Serialize};
use crate::inventory::minion::Minion;
use crate::inventory::super_drill::SuperDrill;

#[derive(Serialize, Deserialize, Clone)]
pub enum InventoryItem {
    Minion(Minion),
    SuperDrill(SuperDrill),
    // Add new items here
}