use saphyr::Yaml;
use anyhow::{Context, Result};

fn main() -> Result<()>{
    let openapi_yaml_raw = include_str!("../assets/openapi.yaml");
    let docs = Yaml::load_from_str(&openapi_yaml_raw)
        .context("Failed to load OpenAPI YAML")?;

    println!("{:#?}", docs);

    // Write it to `assets/openapi.txt` with std::fs
    std::fs::write("assets/openapi.txt", format!("{:#?}", docs))
        .context("Failed to write OpenAPI YAML to file")?;

    Ok(())
}
