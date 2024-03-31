use anyhow::Result;
use hash40::Hash40;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::atomic::AtomicBool;

pub const CONFIG_PATH: &str = "sd:/ultimate/config_title.toml";

#[derive(Serialize, Deserialize)]
pub struct TitleConfig {
    pub ui_bgm_id: Hash40,
    #[serde(default)]
    pub disable_timeout: AtomicBool
}

impl TitleConfig {
    pub fn new() -> TitleConfig {
        TitleConfig {
            ui_bgm_id: Hash40::new("ui_bgm_crs12_simple_result_final"),
            disable_timeout: AtomicBool::new(false),
        }
    }

    pub fn load() -> Result<TitleConfig> {
        if fs::metadata(CONFIG_PATH).is_ok() {
            let toml_str = fs::read_to_string(CONFIG_PATH)?;
            let title_config = toml::from_str::<TitleConfig>(&toml_str)?;
            Ok(title_config)
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound).into())
        }
    }

    pub fn load_or_create() -> Result<TitleConfig> {
        match TitleConfig::load() {
            Ok(c) => Ok(c),
            Err(e) => {
                if e.is::<io::Error>()
                    && e.downcast_ref::<io::Error>().unwrap().kind() == io::ErrorKind::NotFound
                {
                    // No config file exists already
                    TitleConfig::create_default()?;
                    TitleConfig::load()
                } else if e.is::<toml::de::Error>() {
                    // A config file exists but its not in the right format
                    fs::remove_file(CONFIG_PATH)?;
                    TitleConfig::create_default()?;
                    TitleConfig::load()
                } else {
                    // Some other error, re-raise it
                    Err(e)
                }
            }
        }
    }

    /// Creates a default config and saves to file
    /// Returns Err if the file already exists
    pub fn create_default() -> Result<()> {
        if fs::metadata(CONFIG_PATH).is_ok() {
            Err(io::Error::from(io::ErrorKind::AlreadyExists).into())
        } else {
            let default_config: TitleConfig = TitleConfig::new();
            let contents = toml::to_string(&default_config)?;
            fs::write(CONFIG_PATH, contents)?;
            Ok(())
        }
    }
}
