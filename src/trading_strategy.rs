// https://www.investopedia.com/articles/active-trading/101014/basics-algorithmic-trading-concepts-and-examples.asp



use rust_decimal::Decimal;
use alpaca_client::client::{Client, AccountType};
use crate::trading_client::TradingClient;
use alpaca_client::asset::Asset;

pub struct TradeDecision {
    pub name: String,
    pub confidence: Decimal
}

pub struct Stock {
    pub symbol: String,
    pub exchange: String,
}

pub(crate) trait TradingStrategy {
    fn name(&self) -> String;
    fn evaluate_stock(&self, _: Stock) -> TradeDecision;

    fn get_client() -> Client {
        return alpaca_client::client::Client::new("".parse().unwrap(), "".parse().unwrap(), AccountType::PAPER);
    }
}

