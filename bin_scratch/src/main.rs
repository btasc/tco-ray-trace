use latr::{ LatrEngine, LatrConfig, LatrError, PhysicsLoop, Physics };

fn main() -> Result<(), latr::LatrError> {
    let state = SimState {
        x: "hello!",
    };

    let mut config = LatrConfig {
        run_mode: latr::RunMode::Headless,
        physics: Some(Box::new(state))
        ..Default::default()
    };

    let mut engine = LatrEngine::start(config)?;
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

