use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;
use url::Url;

use crate::{error::Result, Error};

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

fn build_addr(ip: IpAddr, url: &Url) -> Result<String> {
    match url.port() {
        Some(port) => Ok(format!("{}:{}", ip, port)),
        None => match url.scheme() {
            "http" => Ok(format!("{}:{}", ip, HTTP_PORT)),
            "https" => Ok(format!("{}:{}", ip, HTTPS_PORT)),
            _ => Err(Error::TcpSchemeNotSupported),
        },
    }
}

pub async fn connect(ip: IpAddr, url: &Url) -> Result<TcpStream> {
    let address = build_addr(ip, url)?;
    TcpStream::connect(address)
        .await
        .map_err(|e| Error::TcpConnect(e.to_string()))
}

pub async fn connect_timeout(ip: IpAddr, url: &Url, timeout: f32) -> Result<TcpStream> {
    let address = build_addr(ip, url)?;

    match tokio::time::timeout(
        Duration::from_secs_f32(timeout),
        TcpStream::connect(address),
    )
    .await
    {
        Ok(s) => Ok(s.map_err(|e| Error::TcpConnect(e.to_string()))?),
        Err(_) => Err(Error::TcpConnectTimeout),
    }
}
