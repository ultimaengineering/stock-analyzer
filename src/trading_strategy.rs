// https://www.investopedia.com/articles/active-trading/101014/basics-algorithmic-trading-concepts-and-examples.asp

struct TradeDecision {
    name: str,
    confidence: float
}

struct Stock {
    symbol: str,
    exchange: str,
}

trait TradingStrategy {
    fn name(&self) -> str;
    fn evaluate_stock(&self, _: Stock) -> TradeDecision;
}