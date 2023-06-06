use crate::DaemonError;
use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;
use typed_builder::TypedBuilder;

#[derive(Copy, Clone, TypedBuilder, Debug)]
pub struct BindSocketRetryOption {
    /// Maximum retry attempts before returning the last occurred error.
    #[builder(default = 10)]
    pub attempts: u8,

    /// Max delay duration before resetting the duration to the
    /// `delay_increment_duration`.
    #[builder(default = Duration::from_secs(40))]
    pub max_delay_duration: Duration,

    /// Duration to add after each failed retry attempt.
    #[builder(default = Duration::from_secs(10))]
    pub increment_duration: Duration,
}

/// Bind port to 0.0.0.0 while enabling address and port re-use.
/// Enabling port re-use means we can bind both old and new instance
/// of a listener to the same ip and port, which means we can run
/// both old and new versions of the proxy in parallel and gradcefully
/// shutdown the old proxy instance after all pending requests have been
/// services.
pub(crate) fn bind_with_addr_and_port_reuse(
    port: u16,
    retry_option: BindSocketRetryOption,
) -> Result<TcpListener, DaemonError> {
    let address = format!("0.0.0.0:{}", port).parse::<SocketAddr>()?;
    let domain = Domain::for_address(address);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;

    retry(|| socket.bind(&(address).into()), retry_option)?;
    socket.set_nonblocking(true)?;
    socket.listen(128)?;
    Ok(TcpListener::from_std(socket.into())?)
}

/// Naive implementation for retrying an operation until
/// it is either successful, or the retries count is 0.
///
/// Retry interval = min(max_delay, previous interval + delay increment)
fn retry<F, T, E>(op: F, mut option: BindSocketRetryOption) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
{
    let mut sleep_duration = Duration::ZERO;

    loop {
        match op() {
            result @ Ok(_) => return result,
            result @ Err(_) => {
                option.attempts -= 1;
                if option.attempts == 0 {
                    return result;
                }

                sleep_duration += option.increment_duration;
                if sleep_duration > option.max_delay_duration {
                    sleep_duration = option.increment_duration;
                }

                sleep(sleep_duration);
            }
        }
    }
}
