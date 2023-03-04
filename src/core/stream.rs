use crate::core::{
    auth::{auth_client, build_auth},
    config::settings,
    error::Error,
    socks::socks_parser,
};
// use crate::core::num::*;
// use std::collections::HashMap;
// use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
// const ;

//本地客户端解析本地的请求
pub async fn handle_client(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Error> {
    match socks_parser(&mut stream).await {
        Ok(data) => {
            println!("handle_client ok: addr: {} data: {:?}", addr, data);
            //远程服务器地址
            let mut server_stream = TcpStream::connect("127.0.0.1:8089").await?;
            //TODO :tls
            //发送auth包
            let auth = build_auth(1, String::from("12345678901234567890123"))?;
            println!("len: {} item: {:?}", auth.len(), auth);
            server_stream.write(&auth).await?;
            //发送socks5包
            server_stream.write(&data).await?;
            println!("client Ready!!!");
            //发送真实请求包同时接受来自server的包
            //连接来自本地client和远端server 的stream
        }
        Err(err) => {
            println!("handle_client addr: {} err: {}", addr, err);
        }
    }
    Ok(())
}

//服务端接收来自本地的连接,校验通过代理请求,校验失败返回网站,全程tls
pub async fn handle_server(mut stream: TcpStream, addr: SocketAddr, conf: &settings) {
    //TODO: tls
    match auth_client(&mut stream, conf).await {
        Ok(addr_dst) => {
            println!("handle_server: auth ok addr: {}", addr);
            match do_server(stream, addr, addr_dst).await {
                Err(err) => {
                    println!("handle_server: do_server err: {}", err);
                }
                _ => {}
            }
        }
        Err(err) => {
            println!("handle_server err: {}", err);
            //TODO: return web data
        }
    }
}

pub async fn do_server(
    mut stream: TcpStream,
    addr: SocketAddr,
    addr_dst: SocketAddr,
) -> Result<(), Error> {
    println!("server Ready!!!");
    println!("start proxy addr: {} addr_dst: {}", addr, addr_dst);
    let mut dst_stream = TcpStream::connect(addr_dst).await?;
    //连接 dst stream 和来自client 的stream
    Ok(())
}

//连接两个stream 两个管道
pub async fn connect(stream_a: TcpStream, stream_b: TcpStream) -> Result<(), Error> {
    Ok(())
}
