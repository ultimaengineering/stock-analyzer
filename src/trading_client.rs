use alpaca_client::client::{AccountType, Client};
use std::process::exit;
use std::env;
use alpaca_client::client::AccountType::{PAPER, LIVE};

pub struct TradingClient {
}

impl TradingClient {
    pub fn new() -> Client {
        return alpaca_client::client::Client::new(self::TradingClient::get_access_key(), self::TradingClient::get_secret_key(), self::TradingClient::get_env());
    }

    fn get_access_key() -> String {
        return match env::var("alpaca_access_key") {
            Ok(key) => key,
            Err(_e) => exit(-1),
        };
    }

    fn get_secret_key() -> String {
        return match env::var("alpaca_secret_key") {
            Ok(key) => key,
            Err(_e) => exit(-1),
        };
    }

    fn get_env() -> AccountType {
        let key = match env::var("alpaca_trading_type") {
            Ok(key) => key,
            Err(_e) => "paper".to_string(),
        };
        let trading_type = match key.to_lowercase().as_str() {
            "paper" => PAPER,
            "live" => LIVE,
            _ => PAPER,
        };
        return trading_type;
    }
}