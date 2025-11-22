

fn main() -> Result<(), latr::LatrError> {
    let mut config = latr::LatrConfig::default();
    config.load_models("ModelConfig.toml".into())?;

    let mut engine = latr::LatrEngine::new(&config)?;

    Ok(())
}