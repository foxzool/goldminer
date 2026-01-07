use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "savedata.txt";

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistentData {
    pub high_score: u32,
    pub high_level: u32,
}

impl PersistentData {
    pub fn load() -> Self {
        let path = Path::new(SAVE_FILE);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => match serde_json::from_str::<PersistentData>(&content) {
                    Ok(data) => {
                        info!("Loaded persistent data from savedata.txt");
                        return data;
                    }
                    Err(e) => {
                        warn!("Failed to deserialize savedata.txt: {}, using defaults", e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read savedata.txt: {}, using defaults", e);
                }
            }
        }
        info!("savedata.txt not found or invalid, using default persistent data");
        Self::default()
    }

    pub fn save(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(SAVE_FILE, content) {
                    error!("Failed to write savedata.txt: {}", e);
                } else {
                    info!("Saved persistent data to savedata.txt");
                }
            }
            Err(e) => {
                error!("Failed to serialize persistent data: {}", e);
            }
        }
    }

    pub fn reset() -> Self {
        if Path::new(SAVE_FILE).exists() {
            if let Err(e) = fs::remove_file(SAVE_FILE) {
                warn!("Failed to remove savedata.txt: {}", e);
            } else {
                info!("Removed savedata.txt");
            }
        }
        Self::default()
    }
}
