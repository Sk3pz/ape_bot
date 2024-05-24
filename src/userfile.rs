use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serenity::all::{Timestamp, UserId};
use std::io::Write;
use crate::hey;
use crate::inventory::Inventory;
use crate::inventory::item::InventoryItem;
use crate::inventory::minion::Minion;
use crate::inventory::super_drill::SuperDrill;

const BASE_PRICE: u64 = 150;
const LEVEL_MULTIPLIER: u64 = 75;
const ASCENSION_COST: u64 = 1_000_000;

#[derive(Deserialize, Serialize, Clone)]
pub struct UserFile {
    pub(crate) level: u16,
    pub(crate) prestige: u16,
    pub(crate) bananas: u64,
    pub(crate) super_nanners: u16,
    pub(crate) ascension: u16,
    pub(crate) mine_tier: u8,

    pub(crate) inventory: Inventory,
}

#[derive(Clone)]
pub struct UserValues {
    pub(crate) id: UserId,
    pub(crate) file: UserFile
}

impl UserValues {
    fn new(id: &UserId) -> Self {
        Self {
            id: id.clone(),
            file: UserFile {
                level: 1,
                prestige: 1,
                bananas: 0,
                super_nanners: 0,
                ascension: 0,
                mine_tier: 0,

                inventory: Inventory {
                    items: Vec::new(),
                }
            }
        }
    }

    pub fn get(id: &UserId) -> Self {
        Self::read(id)
    }

    fn read(id: &UserId) -> Self {
        let raw_path = format!("./users/{}.json", id.get());
        let path = Path::new(raw_path.as_str());

        if !path.exists() {
            Self::generate(id);
            return UserValues::new(id);
        };

        let Ok(data) = fs::read_to_string(path) else {
            Self::generate(id);
            return UserValues::new(id);
        };

        let userfile: UserFile = serde_json::from_str(data.as_str()).expect(format!("failed to deserialize user data with ID {}", id).as_str());

        Self {
            id: id.clone(),
            file: userfile
        }
    }

    fn generate(id: &UserId) {
        let raw_path = format!("./users/{}.json", id.get());
        let path = Path::new(raw_path.as_str());

        if path.exists() {
            hey!("User data already exists: {}", id);
            return;
        };

        let Ok(mut file) = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .open(path) else {
            hey!("Failed to get file for user data: {}", id);
            return;
        };

        let default_file = Self::new(id);

        let Ok(data) = serde_json::to_string(&default_file.file) else {
            hey!("Failed to serialize user data: {}", id.clone());
            return;
        };

        //let default = "{\"level\":1,\"prestige\":1,\"ascension\":0,\"bananas\":0}".to_string();

        if let Err(e) = write!(file, "{}", data) {
            hey!("Failed to write to file for user {}: {}", id, e);
        }
    }

    fn reload(&mut self) {
        *self = Self::read(&self.id);
    }

    fn update(&self) {
        let raw_path = format!("./users/{}.json", self.id.get());
        let path = Path::new(raw_path.as_str());

        if !path.exists() {
            Self::generate(&self.id);
        };

        let Ok(mut file) = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .truncate(true)
            .open(path) else {
            hey!("Failed to get file for user data: {}", &self.id);
            return;
        };

        let Ok(data) = serde_json::to_string(&self.file) else {
            hey!("Failed to serialize user data: {}", &self.id);
            return;
        };

        if let Err(e) = write!(file, "{}", data) {
            hey!("Failed to write to file for user {}: {}", &self.id, e);
        }
    }

    pub fn get_level(&mut self) -> u16 {
        self.reload();
        self.file.level
    }

    pub fn remove_level(&mut self, amt: u16) {
        self.reload();
        self.file.level -= amt;
        self.update();
    }

    pub fn can_levelup(&mut self) -> bool {
        self.reload();
        self.file.bananas >= self.levelup_cost() && self.file.level < 100
    }

    pub fn levelup_cost(&mut self) -> u64 {
        self.reload();
        BASE_PRICE + (self.file.level as u64 * (LEVEL_MULTIPLIER * (self.file.prestige as u64)))
    }

    // returns the new level
    pub fn levelup(&mut self) {
        self.reload();
        self.file.bananas -= self.levelup_cost();
        self.file.level += 1;
        self.update();
    }

    pub fn can_prestige(&mut self) -> bool {
        self.reload();
        self.file.level >= 100 && self.file.prestige < 10
    }

    // returns the new prestige
    pub fn prestige(&mut self) -> u16 {
        self.reload();
        self.file.level = 1;
        self.file.prestige += 1;
        self.update();
        self.file.prestige
    }

    pub fn get_prestige(&mut self) -> u16 {
        self.reload();
        self.file.prestige
    }

    pub fn remove_prestige(&mut self, amt: u16) {
        self.reload();
        self.file.prestige -= amt;
        self.update();
    }

    pub fn get_bananas(&mut self) -> u64 {
        self.reload();
        self.file.bananas
    }

    pub fn remove_bananas(&mut self, bananas: u64) {
        self.reload();
        self.file.bananas -= bananas;
        self.update();
    }

    pub fn add_bananas(&mut self, bananas: u64) {
        self.reload();
        self.file.bananas += bananas;
        self.update();
    }

    pub fn get_ascension(&mut self) -> u16 {
        self.reload();
        self.file.ascension
    }

    pub fn add_ascension(&mut self) {
        self.reload();
        self.file.ascension += 1;
        self.update();
    }

    pub fn remove_ascension(&mut self) {
        self.reload();
        self.file.ascension -= 1;
        self.update();
    }

    pub fn can_ascend(&mut self) -> bool {
        self.reload();
        self.file.prestige >= 10 && self.file.bananas >= ASCENSION_COST
    }

    pub fn ascend(&mut self) {
        self.reload();
        self.file.bananas -= ASCENSION_COST;
        self.file.prestige = 1;
        self.file.level = 1;
        self.file.ascension += 1;
        self.update();
    }

    pub fn get_mine_tier(&mut self) -> u8 {
        self.reload();
        self.file.mine_tier
    }

    pub fn set_mine_tier(&mut self, tier: u8) {
        self.reload();
        self.file.mine_tier = tier;
        self.update();
    }

    pub fn add_super_nanners(&mut self, amt: u16) {
        self.reload();
        self.file.super_nanners += amt;
        self.update();
    }

    pub fn get_super_nanners(&mut self) -> u16 {
        self.reload();
        self.file.super_nanners
    }

    pub fn remove_super_nanners(&mut self, amt: u16) {
        self.reload();
        self.file.super_nanners -= amt;
        self.update();
    }

    pub fn add_inventory_item(&mut self, item: InventoryItem) {
        self.reload();
        self.file.inventory.items.push(item);
        self.update();
    }

    pub fn get_items(&mut self) -> Vec<InventoryItem> {
        self.reload();
        self.file.inventory.items.clone()
    }

    pub fn add_item(&mut self, item: InventoryItem) {
        self.reload();
        self.file.inventory.items.push(item);
        self.update();
    }

    pub fn remove_item(&mut self, item: InventoryItem) {
        self.reload();
        self.file.inventory.items.retain(|i| i != &item);
        self.update();
    }

    pub fn remove_item_index(&mut self, index: usize) {
        self.reload();
        self.file.inventory.items.remove(index);
        self.update();
    }

    pub fn has_super_drill(&mut self) -> bool {
        self.reload();
        self.file.inventory.get_super_drill().is_some()
    }

    pub fn add_super_drill(&mut self) {
        self.reload();
        self.file.inventory.items.push(InventoryItem::SuperDrill(SuperDrill { tier: 1 }));
        self.update();
    }

    pub fn get_super_drill_tier(&mut self) -> u8 {
        self.reload();
        if !self.has_super_drill() {
            0
        } else {
            self.file.inventory.get_super_drill().unwrap().tier
        }
    }

    pub fn get_minions(&mut self) -> Vec<Minion> {
        self.reload();
        self.file.inventory.get_minions()
    }

    pub fn collect_minions(&mut self) -> u64 {
        self.reload();
        // loop through the minions and collect the sludge
        let mut sludge: u64 = 0;
        for minion in &mut self.file.inventory.items {
            if let InventoryItem::Minion(ref mut minion) = minion {
                sludge += minion.get_sludge_produced() as u64;
                minion.mining_start = Timestamp::now();
            }
        }
        // update the minions
        self.update();
        sludge
    }
}

