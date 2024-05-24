use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::inventory::item::InventoryItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct DropTable {
    pub sludge: RangeInclusive<u32>,
    pub super_nanners: Option<RangeInclusive<u32>>,
    pub items: Vec<InventoryItem>,
}

impl DropTable {

    pub fn random_item(&self) -> InventoryItem {
        let mut rng = rand::thread_rng();
        if self.items.len() == 0 {
            return InventoryItem::HealingPotion { health: 10 };
        }
        let index = rng.gen_range(0..self.items.len());
        self.items[index].clone()
    }

}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub name: String,
    pub health: RangeInclusive<u32>,
    pub damage: RangeInclusive<u32>,
    pub reward_scaling: bool,
    pub thumbnail: String,
    pub drops: DropTable,
}

#[derive(Serialize, Deserialize)]
pub struct MineTier {
    pub required_super_drill_tier: u8,
    pub sludge_worth: u32,
    pub super_nanner_chance: f32,
    pub creatures: Vec<Enemy>,
    pub drop_table: DropTable,
}

impl MineTier {

    pub fn random_enemy(&self) -> &Enemy {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.creatures.len());
        &self.creatures[index]
    }

}

pub struct Mine {
    pub tiers: HashMap<u8, MineTier>,
}

impl Mine {

    pub fn get() -> Self {
        // load all mine tiers from ./drop_tables/mine_tiers/*.json
        let mut tiers = HashMap::new();

        // load all mine tiers
        let paths = fs::read_dir("./mine_tiers").unwrap();
        for path in paths {
            // the name of the file should be {mine tier #}.json
            let path = path.unwrap().path();
            let tier_num = path.file_stem().unwrap().to_str().unwrap().parse::<u8>().unwrap();
            let file = fs::read_to_string(path).unwrap();
            let tier: MineTier = serde_json::from_str(&file).unwrap();

            tiers.insert(tier_num, tier);
        }

        Self {
            tiers
        }
    }

    pub fn get_tier(&self, tier: u8) -> &MineTier {
        self.tiers.get(&tier).unwrap()
    }

    // pub fn write(&self) {
    //     for (num, tier) in &self.tiers {
    //         let file = fs::File::create(format!("./mine_tiers/{}.json", num)).unwrap();
    //         serde_json::to_writer(file, tier).unwrap();
    //     }
    // }

}