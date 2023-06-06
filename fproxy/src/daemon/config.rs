use std::sync::Arc;

use trust_dns_resolver::TokioAsyncResolver;
use typed_builder::TypedBuilder;

use crate::config::Apps;
use crate::config::Subscriber;

use super::utils::BindSocketRetryOption;

/// Configuration for proxy daemon.
#[derive(TypedBuilder)]
pub struct DaemonConfig<C> {
    /// DNS resolver.
    pub dns_resolver: Arc<&'static TokioAsyncResolver>,

    /// Handle to listen for  configuration change.
    pub config_subscriber: Subscriber<C, Apps>,

    /// Retry configuration when attempting to bind to host's socket address.
    pub bind_socket_retry_option: BindSocketRetryOption,
}
