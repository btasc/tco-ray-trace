use super::engine_core::Engine;

pub trait PhysicsLoop {
    fn init(&mut self) -> Result<(), LatrError>;
    fn update(&mut self) -> Result<(), LatrError>;
}

// Physics is made to give a nice user handle to the Engine
// This way its a bit easier for the user and they have less things to import
pub struct Physics<T: PhysicsLoop> {
    engine_handle: &mut Engine,
    physics_loop: <T>,
}

impl<T> Physics<T> {
    pub fn new(state: T, engine: &mut Engine) -> Self {
        Physics {
            engine_handle: engine,
            physics_loop: state,
        }
    }
}