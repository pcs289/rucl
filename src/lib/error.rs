use hickory_resolver::error::ResolveError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DNS Resolver: {0}")]
    DNS(String),

    #[error("TCP Connect: {0}")]
    TcpConnect(String),

    #[error("TCP Connection Timeout")]
    TcpConnectTimeout,

    #[error("Scheme not supported")]
    TcpSchemeNotSupported,

    #[error("Native TLS Connector: {0}")]
    TLSNative(String),

    #[error("Tokio TLS Connector: {0}")]
    TLSTokio(String),
}

impl From<ResolveError> for Error {
    fn from(value: ResolveError) -> Self {
        Error::DNS(value.to_string())
    }
}
