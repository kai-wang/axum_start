use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub rule_set: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub rules: Vec<Rule>,
    pub log: Log,
}

const CONFIG_DIRECTORY_NAME: &str = "./conf/";
const DEFAULT_CONFIG_FILE: &str = "./conf/default.toml";

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // get the environment
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());

        let settings = config::Config::builder()
            .add_source(config::File::with_name(DEFAULT_CONFIG_FILE))
            .add_source(config::File::with_name(&format!(
                "{}{}",
                CONFIG_DIRECTORY_NAME, env
            )))
            .build()?
            .try_deserialize()?;

        Ok(settings)
    }
}

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // the environment vaiable is shared in testing, be careful
    #[test]
    fn test_settings_prod() {
        std::env::remove_var("RUN_ENV");
        std::env::set_var("RUN_ENV", "prod");

        println!("the current env {:?}", std::env::var("RUN_ENV"));
        let settings = Settings::new();
        println!("prod settings {:#?}", settings);
    }

    #[test]
    fn test_settings_default() {
        std::env::remove_var("RUN_ENV");

        println!("the current env {:?}", std::env::var("RUN_ENV"));
        let settings = Settings::new();
        println!("default settings {:#?}", settings);
    }
}
