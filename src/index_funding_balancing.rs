 // https://ieeexplore.ieee.org/abstract/document/5586337
use crate::trading_strategy::{TradingStrategy, Stock, TradeDecision};

struct IndexFundBalancing {
    
}

impl TradingStrategy for IndexFundBalancing {
    fn name(&self) -> String {
        return "".to_owned();
    }

    fn evaluate_stock(&self, stock: Stock) -> TradeDecision {
        return TradeDecision {
            name: "".to_string(),
            confidence: Default::default()
        }
    }
}