use std::net::IpAddr;

use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;

use crate::error::{Error, Result};

pub async fn query(host: String) -> Result<IpAddr> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let resolved_ips = resolver.lookup_ip(host).await.map_err(Error::from)?;

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let address = resolved_ips.iter().next().expect("no addresses returned!");
    Ok(address)
}
