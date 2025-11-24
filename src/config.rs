use crate::error::ConfigError;
use serde::Deserialize;
use crate::{LatrEngine, LatrError};

// Config that specifies all settings for running
// Has default implemented so you can just select a few things
// Also contains all the blueprints for different models that we will use
#[derive(Debug)]
pub struct LatrConfig {
    pub fps_cap: u32,
    pub resolution: (u32, u32),
    pub num_rays: (u32, u32),
    pub run_mode: RunMode,
    pub physics_func: Option<fn(&mut LatrEngine) -> Result<(), LatrError>>,

    model_config: Option<ModelConfig>,
}

#[derive(Deserialize, Debug)]
struct ExplicitModelConfig {
    name: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct DirectoriesConfig {
    model_folders: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ModelConfig {
    directories: DirectoriesConfig,
    models: Vec<ExplicitModelConfig>,
}

impl Default for LatrConfig {
    fn default() -> Self {
        let resolution: (u32, u32) = (640, 360);
        let num_rays = resolution;

        Self {
            fps_cap: 60,
            resolution,
            num_rays,
            run_mode: RunMode::default(),
            model_config: None,
            physics_func: None,
        }
    }
}

impl LatrConfig {
    pub fn load_models(&mut self, config_path: std::path::PathBuf) -> Result<(), ConfigError> {
        let toml_string = std::fs::read_to_string(config_path)?;
        let model_config: ModelConfig = toml::from_str(toml_string.as_str())?;

        self.model_config = Some(model_config);
        Ok(())
    }
    
    pub fn load_physics(&mut self, physics_sim: fn(&mut LatrEngine) -> Result<(), LatrError>) {
        self.physics_func = Some(physics_sim);
    }
}

// This is an enum so more can be added later
#[derive(Debug, PartialEq, Default)]
pub enum RunMode {
    #[default] // Gui is now default
    // Gui is the normal run mode with full features and screen
    Gui,

    // Headless is a testing mode that removes the screen, allowing faster testing
    Headless,
}