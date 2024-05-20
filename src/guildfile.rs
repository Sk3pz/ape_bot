use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId};
use crate::hey;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct GuildFile {
    spam_channels: Vec<u64>,
}

pub struct GuildSettings {
    pub id: GuildId,
    pub file: GuildFile,
}

impl GuildSettings {

    pub fn new(guild_id: &GuildId) -> Self {
        Self {
            id: guild_id.clone(),
            file: GuildFile {
                spam_channels: Vec::new(),
            },
        }
    }

    pub fn get(id: &GuildId) -> Self {
        let raw_path = format!("./guilds/{}.json", id);
        let path = std::path::Path::new(&raw_path);

        if !path.exists() {
            Self::generate(id);
            return Self::new(id);
        }

        let Ok(data) = fs::read_to_string(path) else {
            Self::generate(id);
            return Self::new(id);
        };

        let guildfile: GuildFile = serde_json::from_str(data.as_str()).expect(format!("failed to deserialize guild data with ID {}", id).as_str());

        Self {
            id: id.clone(),
            file: guildfile
        }
    }

    fn generate(id: &GuildId) {
        let raw_path = format!("./guilds/{}.json", id.get());
        let path = Path::new(raw_path.as_str());

        if path.exists() {
            hey!("Guild data already exists: {}", id);
            return;
        };

        let Ok(mut file) = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .open(path) else {
            hey!("Failed to get file for guild data: {}", id);
            return;
        };

        let default_file = Self::new(id);

        let Ok(data) = serde_json::to_string(&default_file.file) else {
            hey!("Failed to serialize guild data: {}", id.clone());
            return;
        };

        //let default = "{\"level\":1,\"prestige\":1,\"ascension\":0,\"bananas\":0}".to_string();

        if let Err(e) = write!(file, "{}", data) {
            hey!("Failed to write to file for guild {}: {}", id, e);
        }
    }

    fn reload(&mut self) {
        *self = Self::get(&self.id);
    }

    fn update(&self) {
        let raw_path = format!("./guilds/{}.json", self.id.get());
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
            hey!("Failed to get file for guild data: {}", &self.id);
            return;
        };

        let Ok(data) = serde_json::to_string(&self.file) else {
            hey!("Failed to serialize guild data: {}", &self.id);
            return;
        };

        if let Err(e) = write!(file, "{}", data) {
            hey!("Failed to write to file for guild {}: {}", &self.id, e);
        }
    }

    pub fn get_channels(&mut self) -> Vec<ChannelId> {
        self.reload();
        self.file.spam_channels.clone()
            .iter().map(|x| ChannelId::from(*x)).collect::<Vec<ChannelId>>()
    }

    pub fn add_channel(&mut self, channel_id: u64) {
        self.reload();
        self.file.spam_channels.push(channel_id);
        self.update();
    }

    pub fn remove_channel(&mut self, channel_id: u64) {
        self.reload();
        self.file.spam_channels.retain(|&x| x != channel_id);
        self.update();
    }

    pub fn is_allowed_channel(&mut self, channel_id: u64) -> bool {
        self.reload();
        self.file.spam_channels.contains(&channel_id) || self.file.spam_channels.is_empty()
    }

}