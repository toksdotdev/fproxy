mod config;
mod error;
mod utils;

pub use self::config::DaemonConfig;
pub use self::error::DaemonError;
pub use self::utils::BindSocketRetryOption;
use crate::config::App;
use crate::config::AppConfig;
use crate::config::Port;
use crate::daemon::utils::bind_with_addr_and_port_reuse;
use crate::proxy::Proxy;
use crate::proxy::ProxyConfig;
use crate::strategy::RoundRobinStrategy;
use dashmap::DashMap;
use futures::future::join_all;
use log::as_serde;
use log::info;
use log::warn;
use std::sync::Arc;

/// Process managing proxy and rolling out changes.
pub struct Daemon<C> {
    /// Daemon configuration.
    config: DaemonConfig<C>,
    /// Directory of application proxy context.
    apps: DashMap<App, DashMap<Port, Proxy>>,
}

impl<C> Daemon<C> {
    /// Initialize a instance of the proxy daemon.
    pub fn new(config: DaemonConfig<C>) -> Result<Self, DaemonError> {
        Ok(Self {
            apps: DashMap::new(),
            config,
        })
    }

    /// Start the daemaon process.
    pub async fn start(&mut self) {
        while let Some(config) = self.config.config_subscriber.recv().await {
            let app_update_futures = config
                .apps
                .into_iter()
                .map(|config| self.apply_app_config(config));

            join_all(app_update_futures)
                .await
                .into_iter()
                .for_each(|result| {
                    if let Err(error) = result {
                        // Improvement: Add support for sending events for app & target which failed
                        // which can be rendered to the end-user.
                        warn!("failed to apply configuration {:?}", error)
                    }
                });
        }
    }

    /// Apply application configuration to proxies.
    ///
    /// Improvement: At the moment we're getting the entire config and spinning up new proxy for every
    /// app and killing existing ones if any. Adding support for updating only proxies affected
    /// by the change will be a huge improvement.
    async fn apply_app_config(&self, app_config: AppConfig) -> Result<(), DaemonError> {
        info!(app_name = as_serde!(app_config.name); "applying new configuration");

        // Improvements: Allow users decide the routing strategy from the config.
        // Improvements: If no target is resolved, it'll be good to communicate back to user.
        let strategy = RoundRobinStrategy::new(app_config.targets);
        let retry_option = BindSocketRetryOption::builder().build();
        let target_resolver = Arc::new(strategy);

        if let Some(app) = self.apps.get(&app_config.name) {
            // Drop proxies that do not exist in the new configuration.
            //
            // Improvement(s):
            // - Instead of this, it'll be good to get changes of what happened e.g. port 80 for
            //   app A got deleted, port 9000 for app B was added. That way, we no longer have to
            //   handle the diffing here.
            app.retain(|port, _| app_config.ports.contains(port));
        }

        // Create proxy for newly added app ports.
        for port in app_config.ports {
            let config = ProxyConfig::builder()
                .listener(bind_with_addr_and_port_reuse(port, retry_option)?)
                .dns_resolver(self.config.dns_resolver.clone())
                .target_resolver(target_resolver.clone())
                .build();

            self.apps
                .entry(app_config.name.to_owned())
                .or_default()
                .insert(port, Proxy::listen(config));
        }

        Ok(())
    }
}
