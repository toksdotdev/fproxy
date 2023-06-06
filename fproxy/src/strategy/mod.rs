mod round_robin;

pub use self::round_robin::*;

/// Balancing strategy
pub trait Strategy: Send + Sync {
    type Item;

    /// Get the next item based on a balancing strategy.
    fn next(&self) -> Option<&Self::Item>;
}
