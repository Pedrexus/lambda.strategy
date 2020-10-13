mod rsi;

pub use rsi::RelativeStrengthIndex;
pub use ta::Next;

pub type Error = Box<dyn std::error::Error + 'static>;

pub enum Order {
    Hold,
    Buy,
    Sell,
}

// enum State {
//     Open,  // sold coin
//     Close, // bought coin
// }
