use crate::trading_strategy::{TradingStrategy, TradeDecision};
use crate::trading_strategy::Stock;
use crate::trading_client::TradingClient;
use alpaca_client::client::{Client, AccountType};
use std::env;
use alpaca_client::client::AccountType::PAPER;
use alpaca_client::asset::Asset;
use alpaca_client::bar::{BarRequest, TimeFrame};
use std::process::exit;

pub struct Momentum {

}

impl TradingStrategy for Momentum {
    fn name(&self) -> String {
        return "".to_owned();
    }

    fn evaluate_stock(&self, s: Stock) -> TradeDecision {
        return TradeDecision {
            name: "".to_string(),
            confidence: Default::default()
        }
    }
}

//   Entry Criteria #1: Momentum Day Trading Chart Pattern (Bull Flag or Flat Top Breakout)
//   Entry Criteria #1: Momentum Day Trading Chart Pattern (Bull Flag or Flat Top Breakout)
//   Entry Criteria #2: You have a tight stop that supports a 2:1 profit loss ratio
//   Entry Criteria #3: You have high relative volume (2x or higher) and ideally associated with a catalyst.  Heavier volume means more people are watching.
//   Entry Criteria #4: Low Float is preferred.  I look for under 100mil shares, but under 20million shares is ideal.  You can find the outstanding float with Trade Ideas or eSignal.
fn identify_entry() {
    let assets = get_viable_assets();
    let bar = assets.iter().last()
        .map(|a| {
            let client = TradingClient::new();
            let br = BarRequest {
                time_frame: TimeFrame::OneMinute,
                symbols: a.symbol.to_owned(),
                limit: 0,
                start: None,
                end: None,
                after: None,
                until: None
            };
            return client.get_bar(br);
        });
    println!("{:?}", bar);
    identify_bull_flag_break_out_pattern();
    identify_flat_top_breakout_pattern();
    identify_profit_loss_stop();
    identify_float();
    identify_relative_volume();
    identify_float();
}

// Exit Indicator #1: I will sell 1/2 when profit target met.  If I’m risking $100 to make $200, once I’m up $200 I’ll sell 1/2.  I then adjust my stop to my entry price on the balance of my position
// Exit Indicator #2: If I haven’t already sold 1/2, the first candle to close red is an exit indicator.  If I’ve already sold 1/2, I’ll hold through red candles as long as my breakeven stop doesn’t hit.
// Exit Indicator #3: Extension bar forces me to begin locking in my profits before the inevitable reversal begins.
fn identify_exit() {
    sell_first_half_of_positions();
    sell_on_first_candle();
    sell_on_extension_bar_before_reversal();
}


pub fn identify_bull_flag_break_out_pattern() { // Entry Criteria 1

}

fn identify_flat_top_breakout_pattern() { // Entry Criteria 1

}


fn identify_profit_loss_stop() { //Entry Criteria #2

}

fn identify_relative_volume() { //Entry Criteria #3

}

fn identify_float() {  //Entry Criteria #4

}

fn sell_first_half_of_positions() { //Exit Indicator #1

}

fn sell_on_first_candle() { //Exit Indicator #2

}

fn sell_on_extension_bar_before_reversal() { //Exit Indicator #3

}

fn get_viable_assets() -> Vec<Asset> {
    let client = TradingClient::new();
    let assets = client.get_assets();
    let tradable_assets: Vec<Asset> = assets.iter()
        .filter(|x| x.tradable) // remove assets that are not tradable
        .filter(|x| x.exchange.eq("NASDAQ"))
        .map(|x| x.clone())
        .collect();
    println!("total number of stocks to parse {:?}", tradable_assets.len());
    return tradable_assets;
}




#[cfg(test)]
mod tests {
    use std::process::exit;
    use std::env;
    use alpaca_client::client::Client;
    use alpaca_client::client::AccountType::PAPER;
    use super::*;


    #[test]
    pub fn test_identify_bull_flag_break_out_pattern() {
        identify_entry()
    }
}