use super::error::Error;
use super::ProxyConfig;
use crate::config::TargetAddr;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// A thin client for establishing network connections to
/// a target server with the provided proxy configuration.
///
/// Improvements:
/// - Accept a target client config instead of the entire proxy config.
pub struct TargetClient {
    config: Arc<ProxyConfig>,
}

impl TargetClient {
    pub fn new(config: Arc<ProxyConfig>) -> Self {
        Self { config }
    }

    /// Attempt to connect to all available target based on the balancing strategy.
    ///
    /// For each target, we perform a DNS lookup to get available IPs, and proceed
    /// to connect to all in chunks of 5, and returns with the first TcpStream
    /// that got established.
    ///
    /// Improvement(s):
    /// - Depending on if an app has v4/v6 enabled, filter addresses to connect to by type instead of
    ///   using all the addresses.
    /// - Allow configuring the chunks
    pub async fn connect(&self) -> Result<TcpStream, Error> {
        timeout(self.config.connection_timeout, self.try_connect()).await?
    }

    async fn try_connect(&self) -> Result<TcpStream, Error> {
        while let Some(target) = self.config.target_resolver.next() {
            let addresses = self.lookup(target).await?;

            for addresses in addresses.chunks(self.config.num_parallel_address_connections) {
                let connect_iter = addresses.iter().copied().map(|address| {
                    let config = self.config.clone();
                    async move {
                        let domain = Domain::for_address(address);
                        let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
                        socket.set_tcp_keepalive(&config.keep_alive)?;
                        socket.connect(&(address).into())?;
                        TcpStream::from_std(socket.into())
                    }
                });

                if let Some(stream) = FuturesUnordered::from_iter(connect_iter).try_next().await? {
                    return Ok(stream);
                }
            }
        }

        Err(Error::InvalidAddr)
    }

    async fn lookup(&self, target: &TargetAddr) -> Result<Vec<SocketAddr>, Error> {
        Ok(self
            .config
            .dns_resolver
            .lookup_ip(&format!("{0}.", target.addr))
            .await?
            .iter()
            .map(|ip| SocketAddr::new(ip, target.port))
            .collect())
    }
}
