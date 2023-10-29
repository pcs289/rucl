use tokio::net::TcpStream;
use tokio_native_tls::native_tls;
use tokio_native_tls::{TlsConnector, TlsStream};

use crate::{Error, Result};

pub async fn start_session(
    hostname: String,
    tcp_stream: TcpStream,
) -> Result<TlsStream<TcpStream>> {
    let n_connection =
        native_tls::TlsConnector::new().map_err(|e| Error::TLSNative(e.to_string()))?;

    let connector = TlsConnector::from(n_connection);

    TlsConnector::connect(&connector, &hostname, tcp_stream)
        .await
        .map_err(|e| Error::TLSTokio(e.to_string()))
}
