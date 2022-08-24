use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub environment: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        let port = match env::var("PORT") {
            Ok(port) => port,
            Err(_) => panic!("incorrect port"),
        };

        let port: u16 = port.parse().expect("Can't parse port into number");

        let environment = match env::var("APP_ENV") {
            Ok(environment) => environment,
            Err(_) => panic!("incorrect app_env"),
        };

        Config { port, environment }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config() {
        let c = Config::init();
        assert_eq!(c.port > 0, true);
        assert_eq!(c.environment.chars().count() > 0, true);
    }
}
