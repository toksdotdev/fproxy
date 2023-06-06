use crate::strategy::Strategy;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Round robin strategy which continuously and sequentially cycles 
/// through all items in the underlying haystack without ending.
pub struct RoundRobinStrategy<T> {
    haystack: Vec<T>,
    index: AtomicUsize,
}

impl<T: Send + Sync> RoundRobinStrategy<T> {
    /// Initialize a new instance of the round robbin strategy.
    pub fn new(haystack: Vec<T>) -> Self {
        Self {
            haystack,
            index: AtomicUsize::new(0),
        }
    }
}

impl<T: Send + Sync> Strategy for RoundRobinStrategy<T> {
    type Item = T;

    fn next(&self) -> Option<&Self::Item> {
        let _ = self
            .index
            .compare_exchange(usize::MAX, 0, Ordering::SeqCst, Ordering::SeqCst);

        self.haystack
            .get(self.index.fetch_add(1, Ordering::SeqCst) % self.haystack.len())
    }
}

#[cfg(test)]
mod test {
    use super::RoundRobinStrategy;
    use super::Strategy;

    #[test]
    fn test_round_robin_works() {
        let strategy = RoundRobinStrategy::new(vec![0, 1, 2, 3, 4]);
        assert_eq!(strategy.next(), Some(&0));
        assert_eq!(strategy.next(), Some(&1));
        assert_eq!(strategy.next(), Some(&2));
        assert_eq!(strategy.next(), Some(&3));
        assert_eq!(strategy.next(), Some(&4));
        assert_eq!(strategy.next(), Some(&0));
        assert_eq!(strategy.next(), Some(&1));
        assert_eq!(strategy.next(), Some(&2));
        assert_eq!(strategy.next(), Some(&3));
        assert_eq!(strategy.next(), Some(&4));
    }
}
