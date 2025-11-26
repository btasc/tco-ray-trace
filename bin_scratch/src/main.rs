use latr::{ LatrEngine, LatrConfig };

fn main() -> Result<(), latr::LatrError> {
    let mut config = LatrConfig {
        ..Default::default()
    };

    let state = SimState {
        x: "hello!"
    };

    let mut engine = LatrEngine::new(config)?
        .load_physics(state)
        .start();
}

struct SimState {
    x: &'static str,
}

