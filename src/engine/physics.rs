use super::engine_core::Engine;
use crate::error::LatrError;

pub trait PhysicsLoop {
    fn init(&mut self) -> Result<(), LatrError>;
    fn update(&mut self) -> Result<(), LatrError>;
}

// Physics is made to give a nice user handle to the Engine
// This way its a bit easier for the user and they have less things to import
pub struct Physics<'a> {
    engine_handle: &'a mut Engine,
    physics_loop: Box<dyn PhysicsLoop>,
}

impl Physics<'a> {
    pub fn new(state: Box<dyn PhysicsLoop>, engine: &'a mut Engine) -> Self {
        Physics {
            engine_handle: engine,
            physics_loop: state,
        }
    }
}