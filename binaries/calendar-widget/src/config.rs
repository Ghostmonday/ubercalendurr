use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use directories::UserConfigDir;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub hotkey: HotkeySettings,
    pub api: ApiSettings,
    pub notifications: NotificationSettings,
    pub appearance: AppearanceSettings,
    pub database_path: PathBuf,
    pub deepseek_api_key: String,
    pub debug_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display: DisplaySettings::default(),
            hotkey: HotkeySettings::default(),
            api: ApiSettings::default(),
            notifications: NotificationSettings::default(),
            appearance: AppearanceSettings::default(),
            database_path: PathBuf::from("calendar.db"),
            deepseek_api_key: String::new(),
            debug_mode: false,
        }
    }
}

impl Settings {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            let default = Self::default();
            default.save_to(path)?;
            return Ok(default);
        }

        let content = fs::read_to_string(path)
            .context("Failed to read settings file")?;

        let settings: Settings = toml::from_str(&content)
            .context("Failed to parse settings")?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir()?;
        std::fs::create_dir_all(&config_dir)?;
        let path = config_dir.join("settings.toml");
        self.save_to(&path)
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize settings")?;
        std::fs::write(path, content)
            .context("Failed to write settings file")?;
        Ok(())
    }
}

fn get_config_dir() -> Result<PathBuf> {
    UserConfigDir::ok_or_else(|| anyhow::anyhow!("No config directory"))
        .map(|d| d.to_path_buf())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplaySettings {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub opacity: f64,
    pub always_on_top: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 500,
            height: 400,
            opacity: 0.95,
            always_on_top: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HotkeySettings {
    pub activate: String,
    pub toggle: String,
}

impl Default for HotkeySettings {
    fn default() -> Self {
        Self {
            activate: "Ctrl+Shift+C".to_string(),
            toggle: "Ctrl+Shift+H".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiSettings {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub default_reminder_minutes: u32,
    pub play_sound: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_reminder_minutes: 15,
            play_sound: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub show_hints: bool,
    pub show_speech_hint: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_family: "Cascadia Code".to_string(),
            font_size: 14,
            show_hints: true,
            show_speech_hint: true,
        }
    }
}
