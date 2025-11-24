

fn main() -> Result<(), latr::LatrError> {
    let mut config = latr::LatrConfig::default();
    config.load_models("ModelConfig.toml".into())?;
    config.load_physics(physics_sim);

    let mut engine = latr::LatrEngine::new(config)?;

    engine.start()
}

fn physics_sim(en: &mut latr::LatrEngine) -> Result<(), latr::LatrError> {

    Ok(())
}