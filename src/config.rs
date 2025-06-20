use config::{Config, FileFormat};
use serde::Deserialize;

use crate::Result;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ApiConfig {
    pub(crate) issuer: String,
    pub(crate) public_key: String,
}

pub(crate) fn get_config() -> Result<ApiConfig> {
    let env_source = config::Environment::with_prefix("OIDC_SERVER");
    let conf_file_source = config::File::new("config.toml", FileFormat::Toml).required(false);

    let builder = Config::builder()
        .add_source(conf_file_source)
        .add_source(env_source)
        .build()?;

    let config = builder.try_deserialize()?;

    Ok(config)
}
