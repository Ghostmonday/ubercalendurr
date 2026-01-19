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
mod ipc;

use crate::app::App;
use crate::config::Settings;
use crate::storage::Repository;
use crate::api::DeepSeekClient;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

    app.run()?;

    state.save()?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = directories::UserConfigDir
        .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?
        .join("ubercalendurr");
    
    std::fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir.join("settings.toml"))
}

#[derive(Clone)]
struct AppState {
    settings: Arc<Settings>,
    repository: Arc<Repository>,
    deepseek_client: Arc<DeepSeekClient>,
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
        let repository = Arc::new(Repository::new(&settings.database_path)?);
        let deepseek_client = Arc::new(DeepSeekClient::new(&settings.deepseek_api_key)?);

        Ok(Self {
            settings: Arc::new(settings.clone()),
            repository,
            deepseek_client,
            input_buffer: Arc::new(std::sync::RwLock::new(String::new())),
            processing_state: Arc::new(std::sync::RwLock::new(ProcessingState::Idle)),
        })
    }

    fn save(&self) -> Result<()> {
        self.settings.save()
    }
}
