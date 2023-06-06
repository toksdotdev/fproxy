use notify::Error as NotifyError;
use std::error::Error;

#[derive(Debug)]
pub enum WatcherError {
    NotifyError(NotifyError),
    Other(Box<dyn Error>),
}

impl From<NotifyError> for WatcherError {
    fn from(e: NotifyError) -> Self {
        Self::NotifyError(e)
    }
}

impl From<Box<dyn Error>> for WatcherError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::Other(e)
    }
}
