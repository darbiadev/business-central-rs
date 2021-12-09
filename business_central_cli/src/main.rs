use std::error::Error;
use std::path::PathBuf;

use dirs::config_dir;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use reqwest::{Method, StatusCode};
use serde::Deserialize;
use serde_json::Value;

use business_central::call;

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    base_url: String,
    tenant_id: String,
    environment: String,
    company_name: String,
    username: String,
    web_service_access_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_toml_path: PathBuf = [
        config_dir().unwrap().to_str().unwrap(),
        "darbia",
        "bcrs.toml",
    ]
        .iter()
        .collect();
    let config: Config = Figment::new()
        .merge(Toml::file(config_toml_path))
        .merge(Env::prefixed("BCRS_"))
        .extract()?;

    println!("{:#?}", config);

    let response: Result<Value, StatusCode> = call(
        config.base_url,
        config.tenant_id,
        config.environment,
        config.company_name,
        config.username,
        config.web_service_access_key,
        Method::GET,
        "/".to_string(),
    )
        .await;
    println!("{}", response.unwrap_err());

    Ok(())
}
