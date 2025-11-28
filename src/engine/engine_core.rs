use crate::error::EngineError;
use crate::config::LatrConfig;
use crate::{LatrEngine, LatrError};

// These return LatrErrors as the user will run most physics operations through the LatrEngine api
// This way the user doesn't have to be like "engine.engine.run_op", and doesn't have to deal with EngineError vs LatrError

pub struct Engine {
    
}

impl Engine {
    pub fn new(config: &LatrConfig) -> Self {
        Self {}
    }
}