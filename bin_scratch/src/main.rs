use latr::{ LatrEngine, LatrConfig, PhysicsLoop, Physics };

fn main() -> Result<(), latr::LatrError> {
    let mut config = LatrConfig {
        run_mode: latr::RunMode::Headless,
        physics: Some(SimState)
        ..Default::default()
    };

    let state = SimState {
        x: "hello!"
    };

    let physics = Physics::new();

    let mut engine = LatrEngine::new(config)?
        .load_physics(Box::new(state))
        .start();
}

struct SimState {
    x: &'static str,
}

impl PhysicsLoop for SimState {
    fn init(&mut self, phys: &mut Physics) -> Result<(), LatrError> {

    }

    fn update(&mut self, phys: &mut Physics) -> Result<(), LatrError> {

    }
}

