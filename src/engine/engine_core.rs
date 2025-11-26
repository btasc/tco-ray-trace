use crate::error::EngineError;
use crate::config::LatrConfig;
use crate::{LatrEngine, LatrError};

// These return LatrErrors as the user will run most physics operations through the LatrEngine api
// This way the user doesn't have to be like "engine.engine.run_op", and doesn't have to deal with EngineError vs LatrError
pub trait PhysicsLoop {
    fn init(&mut self) -> Result<(), LatrError>;
    fn update(&mut self) -> Result<(), LatrError>;
}

pub struct Engine {
    pub physics_loop: Option<Box<dyn PhysicsLoop>>,
}

impl Engine {
    pub fn new(config: &LatrConfig, physics_loop: Option<Box<dyn PhysicsLoop>>) -> Result<Self, EngineError> {

        Ok(Self {
            physics_loop,
        })
    }

    pub fn run_physics(&self, latr_engine: LatrEngine) -> Result<(), LatrError> {

    }
}