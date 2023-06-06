use tokio::time::error::Elapsed;
use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(std::io::Error),

    /// Failed to lookup DNS record for IP address.
    DnsLookup(ResolveError),

    /// Socket address is invalid or couldn't be resolved.
    InvalidAddr,

    /// Failed to establish connection to the target before the
    /// conection timeout exceeded.
    ConnectionTimeout,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ResolveError> for Error {
    fn from(e: ResolveError) -> Self {
        Self::DnsLookup(e)
    }
}

impl From<Elapsed> for Error {
    fn from(_: Elapsed) -> Self {
        Self::ConnectionTimeout
    }
}
