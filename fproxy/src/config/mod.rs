mod error;
mod schema;
mod subscribers;

pub use self::error::*;
pub use self::schema::*;
pub use self::subscribers::*;
use tokio::sync::mpsc::UnboundedReceiver;

/// Watch for configuration changes.
pub trait ConfigSubscriber<C> {
    /// Error that occurs while subscribing for config changes.
    type Error;

    /// Most recent configuration after the chnage occurs.
    type Config;

    /// Subscribe to changes in a configuration source.
    fn subscribe(&self) -> Result<Subscriber<C, Self::Config>, Self::Error>;
}

/// Handle to a subscribe to configuration changes.
pub struct Subscriber<C, T> {
    /// Handle context.
    #[allow(dead_code)]
    context: C,

    /// Receiver for configuration changes.
    rx: UnboundedReceiver<T>,
}

impl<C, T> Subscriber<C, T> {
    /// Listen for the next configuration changes. `None` will
    /// only be returned when the underlying channel has been closed
    /// and all received messages have been processed.
    pub async fn recv(&mut self) -> Option<T> {
        self.rx.recv().await
    }
}
