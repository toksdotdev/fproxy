use crate::config::TargetAddr;
use crate::strategy::Strategy;
use socket2::TcpKeepalive;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use trust_dns_resolver::TokioAsyncResolver;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ProxyConfig {
    /// DNS resolver
    pub dns_resolver: Arc<&'static TokioAsyncResolver>,

    /// Underlying TCP listener.
    pub listener: TcpListener,

    /// Strategy for resolving what target to connect to.
    pub target_resolver: Arc<dyn Strategy<Item = TargetAddr>>,

    /// Max time to wait for graceful shutdown.
    ///
    /// Default value: 10 seconds
    #[builder(default = Duration::from_millis(10000))]
    pub shutdown_timeout: Duration,

    /// Delay duration before retrying shutdown after previous attempt failed.
    ///
    /// This should always be less than the `shutdown_timeout` which is the max
    /// time to wait.
    ///
    /// Default value: 100 milliseconds
    #[builder(default = Duration::from_millis(100))]
    pub shutdown_retry_delay: Duration,

    /// The maximum time to wait for a network connection to be
    /// established with target, in milliseconds.
    ///
    /// Default value: 30 seconds
    #[builder(default = Duration::from_millis(30000))]
    pub connection_timeout: Duration,

    /// Buffer size for the signal channel.
    #[builder(default = 5)]
    pub signal_buffer_size: usize,

    /// Keep alive configuration for the proxy
    #[builder(
        default = TcpKeepalive::new()
            .with_interval(Duration::from_secs(75))
            .with_time(Duration::from_secs(10))
            .with_retries(9)
    )]
    pub keep_alive: TcpKeepalive,

    /// The number of lookup addresses to attempt connecting to in parallel.
    #[builder(default = 5)]
    pub num_parallel_address_connections: usize,
}

impl Debug for ProxyConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProxyConfig")
            .field("listener", &self.listener)
            .field("shutdown_timeout", &self.shutdown_timeout)
            .field("shutdown_retry_delay", &self.shutdown_retry_delay)
            .field("signal_buffer_size", &self.signal_buffer_size)
            .field("connection_timeout", &self.connection_timeout)
            .field("keep_alive", &self.keep_alive)
            .field(
                "num_parallel_address_connections",
                &self.num_parallel_address_connections,
            )
            .finish()
    }
}
