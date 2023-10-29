use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;

use crate::{error::Result, Error};

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

fn build_addr(ip: IpAddr, scheme: &str) -> Result<String> {
    match scheme {
        "http" => Ok(format!("{}:{}", ip, HTTP_PORT)),
        "https" => Ok(format!("{}:{}", ip, HTTPS_PORT)),
        _ => Err(Error::TcpSchemeNotSupported),
    }
}

pub async fn connect(ip: IpAddr, scheme: &str) -> Result<TcpStream> {
    let address = build_addr(ip, scheme)?;
    TcpStream::connect(address)
        .await
        .map_err(|e| Error::TcpConnect(e.to_string()))
}

pub async fn connect_timeout(ip: IpAddr, scheme: &str, timeout: f32) -> Result<TcpStream> {
    let address = build_addr(ip, scheme)?;

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
