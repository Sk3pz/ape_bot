use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use serde::{Deserialize, Serialize};
use crate::inventory::minion::Minion;
use crate::inventory::super_drill::SuperDrill;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum WeaponType {
    Sword,
    Bow,
    Stick,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Sword => write!(f, "Sword"),
            WeaponType::Bow => write!(f, "Bow"),
            WeaponType::Stick => write!(f, "Stick"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum InventoryItem {
    Minion(Minion),
    SuperDrill(SuperDrill),
    HealingPotion { health: u32 },
    SpellTome { name: String, damage: RangeInclusive<u32> },
    Weapon { name: String, wtype: WeaponType, damage: RangeInclusive<u32> },
}

impl Display for InventoryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryItem::Minion(_) => write!(f, "Minion"),
            InventoryItem::SuperDrill(drill) => write!(f, "Super Drill ({})", drill.tier),
            InventoryItem::HealingPotion { health: max_effectiveness } => write!(f, "Healing Potion ({}hp)", max_effectiveness),
            InventoryItem::SpellTome {
                name, ..
            } => write!(f, "{} Tome", name),
            InventoryItem::Weapon {
                name, wtype, ..
            } => write!(f, "{} ({})", name, wtype),
        }
    }
}