use std::io::Read;

use serde::{Deserialize, Serialize};
use crate::util::types::User;

use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub user: Option<User>,
    pub biz_id: Option<String>, // user info don't actually allow to do too much
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("whop")
            .join("config.toml");

        if !config_path.exists() {
            Self {
                user: None,
                biz_id: None
            }
        } else {
            let mut config_file = std::fs::File::open(config_path).unwrap();

            let mut buf = String::new();

            config_file
                .read_to_string(&mut buf)
                .unwrap();

            toml::from_str(&buf).unwrap()
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("whop")
            .join("config.toml");

        let config_dir = config_path.parent().unwrap();
        
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }

        let config_file = std::fs::File::create(config_path).unwrap();

        let config_data = toml::to_string_pretty(self).unwrap();

        std::io::Write::write_all(&mut &config_file, config_data.as_bytes()).unwrap();

        Ok(())
    }

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn set_biz_id(&mut self, biz_id: String) {
        self.biz_id = Some(biz_id)
    }

    pub fn get_user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn get_biz_id(&self) -> Option<&String> {
        self.biz_id.as_ref()
    }

    pub fn delete_user(&mut self) {
        self.user = None;
    }

    pub fn delete_biz_id(&mut self) {
        self.biz_id = None;
    }
}