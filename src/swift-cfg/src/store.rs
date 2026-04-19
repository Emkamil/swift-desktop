// Copyright (C) 2026  Kamil Machowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ConfigStorage {
    #[serde(flatten)]
    pub categories: HashMap<String, HashMap<String, String>>,
}

pub struct Store {
    pub data: RwLock<ConfigStorage>,
    user_path: PathBuf,
}

impl Store {
    pub fn new() -> Self {
        let home = std::env::var("HOME").expect("Warning: HOME environment variable is missing.");
        let user_path = PathBuf::from(home).join(".config/swift/settings.toml");
        let system_path = PathBuf::from("/usr/share/swift/defaults.toml");

        let mut final_config = ConfigStorage::default();

        // 1. Ładowanie systemowych domyślnych
        if let Some(sys_data) = Self::load_file(&system_path) {
            final_config.categories = sys_data.categories;
        } else {
            eprintln!("Warning: Default configuration not found in /usr/share/swift/defaults.toml");
        }

        // 2. Ładowanie i scalanie ustawień użytkownika (Deep Merge)
        if let Some(user_data) = Self::load_file(&user_path) {
            for (category_name, settings) in user_data.categories {
                let cat = final_config.categories.entry(category_name).or_insert_with(HashMap::new);
                for (key, value) in settings {
                    cat.insert(key, value);
                }
            }
        }

        Self {
            data: RwLock::new(final_config),
            user_path,
        }
    }

    fn load_file(path: &PathBuf) -> Option<ConfigStorage> {
        if !path.exists() { return None; }
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).map_err(|e| eprintln!("TOML Error in {:?}: {}", path, e)).ok()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let db = self.data.read().unwrap();
        let toml_str = toml::to_string_pretty(&*db).unwrap();
        if let Some(parent) = self.user_path.parent() { fs::create_dir_all(parent)?; }
        fs::write(&self.user_path, toml_str)
    }
}