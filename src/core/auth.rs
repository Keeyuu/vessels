use crate::core::{
    config::settings,
    error::Error,
    num::{u64_to_u8, u8_to_u64},
    socks::{get_socks_addr, socks_parser},
};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
const AUTH_LEN: usize = 32;

//已经经过tls所以数据是安全的直接拿即可
pub async fn auth_client(
    stream: &mut TcpStream,
    //addr: SocketAddr,
    conf: &settings,
) -> Result<SocketAddr, Error> {
    let mut buffer = [0; AUTH_LEN];
    let size = stream.read(&mut buffer).await?;
    if size != AUTH_LEN {
        return Err(Error::new(format!(
            "auth_client: size!={}: {}",
            AUTH_LEN, size
        )));
    }
    parser_auth(buffer, conf)?;
    //校验成功 获取代理请求
    let mut buffer = [0; 256];
    let size = stream.read(&mut buffer).await?;
    if size < 7 {
        return Err(Error::new(format!(
            "socks_parser: read size < 7 size: {}",
            size
        )));
    }
    if buffer[0] != 0x05 || buffer[1] != 0x01 || buffer[2] != 0x00 {
        return Err(Error::new(format!(
            "socks_parser: socks version: {} proxy type: {} RSV: {}",
            buffer[0], buffer[1], buffer[2]
        )));
    }
    let addr = get_socks_addr(&buffer[..size])?;
    Ok(addr)
}

//|version 1|timestamp 8个u8 -> 64| secret|
fn parser_auth(buffer: [u8; AUTH_LEN], conf: &settings) -> Result<(), Error> {
    if buffer[0] != conf.version {
        return Err(Error::new(format!(
            "auth_client: version!=conf: {}",
            buffer[0]
        )));
    }
    if SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        > u8_to_u64(&buffer[1..9])? + conf.timeout
    {
        return Err(Error::new(format!("auth_client: shakehands timeout")));
    }
    if conf.password != String::from_utf8((&buffer[9..]).to_vec())? {
        return Err(Error::new(format!("auth_client: password!!!")));
    }
    Ok(())
}

pub fn build_auth(version: u8, passwd: String) -> Result<[u8; AUTH_LEN], Error> {
    let mut buffer = [0; AUTH_LEN];
    buffer[0] = version;
    let ts_vec = u64_to_u8(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())?;
    if ts_vec.len() != 8 {
        return Err(Error::new(format!("build_auth: u64_to_u8 !=8")));
    }
    for (k, v) in ts_vec.iter().enumerate() {
        buffer[k + 1] = *v;
    }
    for (k, v) in passwd.as_bytes().iter().enumerate() {
        buffer[k + 9] = *v;
    }
    Ok(buffer)
}
