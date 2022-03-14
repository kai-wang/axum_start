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
mod test_settings {
    use super::*;

    // the environment vaiable is shared in testing, be careful
    #[test]
    fn test_settings_prod() {
        std::env::remove_var("RUN_ENV");
        std::env::set_var("RUN_ENV", "prod");

        assert_eq!(std::env::var("RUN_ENV").unwrap(), "prod");
        let settings = Settings::new();
        assert_eq!(settings.unwrap().server.url.contains("prod"), true);
    }

    // #[test]
    // fn test_settings_dev() {
    //     std::env::remove_var("RUN_ENV");
    //     std::env::set_var("RUN_ENV", "dev");

    //     assert_eq!(std::env::var("RUN_ENV").unwrap(), "dev");
    //     let settings = Settings::new();
    //     assert_eq!(settings.unwrap().server.url.contains("dev"), true);
    // }
}
