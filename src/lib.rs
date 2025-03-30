use config::Config;
use runtime::Runtime;

use serde_yaml;
use std::fs;
mod config;
mod runtime;
mod worker_pool;
pub use runtime::Response;

pub fn read_config() -> Config {
    let config_content = fs::read_to_string("config.yml").expect("Failed to read config file");
    let config: config::Config =
        serde_yaml::from_str(&config_content).expect("Failed to parse YAML");
    return config;
}

pub fn start_runtime() -> Runtime {
    let config = read_config();
    Runtime::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", read_config())
    }
}
