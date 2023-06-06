use std::sync::Arc;

use fproxy::dns::default_async_dns_resolver;
use fproxy::BindSocketRetryOption;
use fproxy::ConfigFileSubscriber;
use fproxy::ConfigSubscriber;
use fproxy::Daemon;
use fproxy::DaemonConfig;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config_path =
        env::var("FPROXY_CONFIG_PATH").expect("missing env variable `FPROXY_CONFIG_PATH`");
    let config_subscriber = ConfigFileSubscriber::new(&config_path)
        .subscribe()
        .expect(&format!(
            "failed to subscribe to changes in `{config_path}`"
        ));

    let dns_resolver = default_async_dns_resolver()
        .await
        .expect("failed to load default dns resolver");

    let daemon_config = DaemonConfig::builder()
        .config_subscriber(config_subscriber)
        .bind_socket_retry_option(BindSocketRetryOption::builder().build())
        .dns_resolver(Arc::new(dns_resolver))
        .build();

    Daemon::new(daemon_config)
        .expect("failed to startup daemon")
        .start()
        .await;
}
