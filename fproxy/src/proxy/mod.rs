mod client;
mod config;
pub mod error;

pub use self::config::*;
use crate::proxy::client::TargetClient;

use log::as_serde;
use log::debug;
use log::error;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Instant;
use tokio::io::copy_bidirectional;
use tokio::spawn;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tracing::instrument;

/// Signal type supported by proxy.
pub enum Signal {
    /// Shutdown proxy gracefully.
    SIGTERM,
}

/// Proxy between a listener and multiple target.
///
/// What target a proxy connects is dependeny on the strategy
/// configured for the target resolver.
#[derive(Debug)]
pub struct Proxy {
    tx: Sender<Signal>,
    request_handler: JoinHandle<()>,
    config: Arc<ProxyConfig>,
}

impl Proxy {
    /// Start proxying request from the provided TCP listener.
    pub fn listen(config: ProxyConfig) -> Self {
        let config = Arc::new(config);
        let (tx, rx) = channel::<Signal>(config.signal_buffer_size);
        let request_handler = spawn(Self::handle_requests(config.clone(), rx));

        Self {
            tx,
            config,
            request_handler,
        }
    }

    /// Process an incoming TCP stream.
    #[instrument(skip(signal_rx, config))]
    async fn handle_requests(config: Arc<ProxyConfig>, mut signal_rx: Receiver<Signal>) {
        loop {
            let config = config.clone();

            tokio::select! {
                Some(Signal::SIGTERM) = signal_rx.recv() => break,
                Ok((mut incoming, _)) = config.listener.accept() => {
                    spawn(async move {
                        let Ok(mut target) = TargetClient::new(config).connect().await else {
                            return;
                        };

                        let address = target.peer_addr().map(|addr| addr.to_string()).unwrap_or_default();
                        if let Err(error) = copy_bidirectional(&mut incoming, &mut target).await {
                            debug!(destination = as_serde!(address); "write to target failed: {}", error);
                        }
                    });
                }
            }
        }
    }

    /// Shutdown proxy synchronously.
    pub fn shutdown(&self) {
        if let Err(error) = self.tx.try_send(Signal::SIGTERM) {
            error!("failed to send shutdown signal: {error}");
            self.request_handler.abort();
            return;
        }

        let begin = Instant::now();
        let Some(end) = begin.checked_add(self.config.shutdown_timeout) else {
            error!("shutdown timer exceeded possible range");
            self.request_handler.abort();
            return;
        };

        while begin < end {
            if self.request_handler.is_finished() {
                return;
            }

            sleep(self.config.shutdown_retry_delay);
        }

        self.request_handler.abort();
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        self.shutdown();
    }
}
