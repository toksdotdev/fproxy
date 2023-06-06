use std::error::Error;
use std::fs;
use std::path::Path;

use crate::config::error::WatcherError;
use crate::config::schema::Apps;
use crate::config::ConfigSubscriber;
use crate::config::Subscriber;
use log::debug;
use log::error;
use notify::event::DataChange;
use notify::event::ModifyKind;
use notify::Config;
use notify::Error as NotifyError;
use notify::Event;
use notify::EventKind;
use notify::FsEventWatcher;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use tokio::sync::mpsc::unbounded_channel;

/// File watcher context
pub struct FileContext(FsEventWatcher);

/// File watcher.
pub struct ConfigFileSubscriber<P: AsRef<Path>>(P);

impl<P: AsRef<Path>> ConfigFileSubscriber<P> {
    /// Create a new instance of the config file watcher.
    pub fn new(path: P) -> Self {
        Self(path)
    }
}

impl<P: AsRef<Path>> ConfigSubscriber<FileContext> for ConfigFileSubscriber<P> {
    type Error = WatcherError;
    type Config = Apps;

    fn subscribe(&self) -> Result<Subscriber<FileContext, Self::Config>, Self::Error> {
        let (tx, rx) = unbounded_channel();
        tx.send(fetch_config(&self.0)?)
            .map_err(|e| WatcherError::Other(Box::new(e)))?;

        let event_handler = move |result: Result<Event, NotifyError>| {
            futures::executor::block_on(async {
                match result {
                    Err(err) => error!("{}", err),
                    Ok(event) => {
                        if EventKind::Modify(ModifyKind::Data(DataChange::Content)) != event.kind {
                            return;
                        }

                        let Some(path) = event.paths.first() else {
                            debug!("config event doesn't have file path");
                            return;
                        };

                        debug!("received config event for: ({path:?})");
                        match fs::read_to_string(path) {
                            Err(err) => error!("{}", err),
                            Ok(content) => match serde_json::from_str(&content) {
                                Err(err) => error!("failed to parse config file: {}", err),
                                Ok::<Apps, _>(config) => {
                                    if let Err(err) = tx.send(config) {
                                        error!("failed to send config to receiver: {}", err);
                                    }
                                }
                            },
                        };
                    }
                }
            })
        };

        let mut watcher = RecommendedWatcher::new(event_handler, Config::default())?;
        watcher.watch(self.0.as_ref(), RecursiveMode::Recursive)?;

        Ok(Subscriber {
            context: FileContext(watcher),
            rx,
        })
    }
}

fn fetch_config<P: AsRef<Path>>(path: P) -> Result<Apps, Box<dyn Error>> {
    fs::read_to_string(path)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
        .and_then(|content| {
            serde_json::from_str(&content).map_err(|e| Box::new(e) as Box<dyn Error>)
        })
}
