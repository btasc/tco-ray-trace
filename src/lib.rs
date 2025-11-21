pub mod error;
mod engine;
mod gpu_utils;
mod event_loop;
mod latr_core;
mod config;

use crate::{
    error::LatrError,
    latr_core::LatrEngine,
    config::LatrConfig,
};

pub fn main_func() -> Result<(), LatrError> {
    let mut config = LatrConfig::default();
    config.load_models("../ModelConfig.toml".into())?;

    let engine = LatrEngine::new(&config)?;

    Ok(())
}