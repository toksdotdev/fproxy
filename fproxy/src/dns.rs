use once_cell::sync::OnceCell;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::error::ResolveError;
use trust_dns_resolver::TokioAsyncResolver;

static ASYNC_RESOLVER: OnceCell<TokioAsyncResolver> = OnceCell::new();

/// Max number of DNS records to cache.
///
/// Improvemenets:
/// - Make this configurable.
const CACHE_SIZE: usize = 1000;

/// Default async DNS resolver to use as default
pub async fn default_async_dns_resolver() -> Result<&'static TokioAsyncResolver, ResolveError> {
    ASYNC_RESOLVER.get_or_try_init(|| {
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), build_options())
    })
}

fn build_options() -> ResolverOpts {
    let mut options = ResolverOpts::default();
    options.cache_size = CACHE_SIZE;
    options
}
