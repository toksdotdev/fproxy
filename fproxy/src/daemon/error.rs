use crate::WatcherError;
use std::io::Error as IoError;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum DaemonError {
    ConfigWatcher(WatcherError),
    ParseAddr(AddrParseError),
    IoError(IoError),
}

impl From<WatcherError> for DaemonError {
    fn from(error: WatcherError) -> Self {
        Self::ConfigWatcher(error)
    }
}

impl From<AddrParseError> for DaemonError {
    fn from(error: AddrParseError) -> Self {
        Self::ParseAddr(error)
    }
}

impl From<IoError> for DaemonError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}
