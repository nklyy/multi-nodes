use dotenv::dotenv;
use std::env;

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub environment: String,

    pub bitcoin_rpc_config: BitcoinRpcConfig,
}

#[derive(Default, Debug, Clone)]
pub struct BitcoinRpcConfig {
    pub bitcoin_rpc_user: String,
    pub bitcoin_rpc_password: String,
    pub bitcoin_rpc_url_one: String,
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

        let bitcoin_rpc_user = match env::var("BITCOIN_RPC_USER") {
            Ok(environment) => environment,
            Err(_) => panic!("incorrect bitcoin rpc user"),
        };

        let bitcoin_rpc_password = match env::var("BITCOIN_RPC_PASSWORD") {
            Ok(environment) => environment,
            Err(_) => panic!("incorrect bitcoin rpc password"),
        };

        let bitcoin_rpc_url_one = match env::var("BITCOIN_RPC_URL") {
            Ok(environment) => environment,
            Err(_) => panic!("incorrect bitcoin rpc url"),
        };

        Config {
            port,
            environment,
            bitcoin_rpc_config: BitcoinRpcConfig {
                bitcoin_rpc_user,
                bitcoin_rpc_password,
                bitcoin_rpc_url_one,
            },
        }
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
