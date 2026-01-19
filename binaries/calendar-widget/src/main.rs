#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

mod app;
mod config;
mod storage;
mod api;
mod input;
mod ui;
mod export;
mod notifications;

use crate::app::App;
use crate::config::Settings;
use crate::storage::Repository;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("Starting UberCalendurr Widget {}", env!("CARGO_PKG_VERSION"));

    let config_path = get_config_path()?;
    let settings = Settings::load(&config_path)
        .context("Failed to load settings")?;

    let state = Arc::new(AppState::new(&settings)?);

    let mut app = App::new(state.clone())?;

    info!("UberCalendurr Widget initialized successfully");

    let rt = tokio::runtime::Runtime::new()
        .context("Failed to create tokio runtime")?;
    rt.block_on(app.run())?;

    state.save()?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = directories::BaseDirs::new()
        .ok_or_else(|| anyhow::anyhow!("No config directory"))?
        .config_dir()
        .join("ubercalendurr");
    
    std::fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir.join("settings.toml"))
}

#[derive(Clone)]
struct AppState {
    settings: Arc<Settings>,
    repository: Repository,
    deepseek_client: Option<Arc<deepseek_client::DeepSeekClient>>,
    notification_service: Arc<notifications::NotificationService>,
    input_buffer: Arc<std::sync::RwLock<String>>,
    processing_state: Arc<std::sync::RwLock<ProcessingState>>,
}

#[derive(Debug, Clone, PartialEq)]
enum ProcessingState {
    Idle,
    Processing,
    Complete,
    Error(String),
}

impl AppState {
    fn new(settings: &Settings) -> Result<Self> {
        let repository = Repository::new(&settings.database_path)?;
        
        // Make AI client optional - app works without API key
        let deepseek_client = if !settings.deepseek_api_key.is_empty() {
            let config = deepseek_client::DeepSeekConfig {
                api_key: settings.deepseek_api_key.clone(),
                ..Default::default()
            };
            match deepseek_client::DeepSeekClient::new(config) {
                Ok(client) => {
                    info!("DeepSeek client initialized successfully");
                    Some(Arc::new(client))
                }
                Err(e) => {
                    info!("DeepSeek client initialization failed: {}. Continuing without AI.", e);
                    None
                }
            }
        } else {
            info!("No DeepSeek API key provided. Using SimpleParser only.");
            None
        };

        // Initialize notification service
        let notification_service = Arc::new(notifications::NotificationService::new(
            settings.notifications.enabled,
            settings.notifications.play_sound,
        ));
        
        Ok(Self {
            settings: Arc::new(settings.clone()),
            repository,
            deepseek_client,
            notification_service,
            input_buffer: Arc::new(std::sync::RwLock::new(String::new())),
            processing_state: Arc::new(std::sync::RwLock::new(ProcessingState::Idle)),
        })
    }

    fn save(&self) -> Result<()> {
        self.settings.save()
    }
}
