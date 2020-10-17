mod rsi;

pub use rsi::RelativeStrengthIndex;
pub use ta::Next;
use market_finance::Bar;

pub type Error = Box<dyn std::error::Error + 'static>;

#[derive(Debug)]
pub enum Order {
    Hold,
    Buy,
    Sell,
}

// enum State {
//     Open,  // sold coin
//     Close, // bought coin
// }

pub trait Strategy: Next<f64, Output = Order> {
    fn evaluate(&mut self, data: Vec<Bar>) -> Vec<Order> {
        data.into_iter().map(|v| self.next(v.close)).collect()
    }
}
