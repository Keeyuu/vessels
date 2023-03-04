use crate::core::error::Error;
// use crate::core::num::*;
// use std::collections::HashMap;
// use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
// const ;

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    let stream = socks_parser(stream)?;
    Ok(())
}

fn socks_parser(mut stream: TcpStream) -> Result<TcpStream, Error> {
    let mut buffer = [0; 256 + 2];
    // +-----+----------+----------+
    // | VER | NMETHODS | METHODS  |
    // +-----+----------+----------+
    // |  1  |    1     | 1 to 255 |
    // +-----+----------+----------+
    let size = stream.read(&mut buffer)?;
    if size > 256 + 2 {
        return Err(Error::new(format!("socks_parser: size: {}", size)));
    }
    if buffer[0] != 0x05 {
        return Err(Error::new(format!(
            "socks_parser: socks type: {}",
            buffer[0]
        )));
    }
    let nmethods = buffer[1];
    if nmethods as usize != size - 2 {
        return Err(Error::new(format!(
            "socks_parser: socks nmethods: {} has read methods: {}",
            nmethods,
            size - 2
        )));
    }
    println!("支持的协议版本: {:?}", &buffer[2..size]);
    //TODO : 协议校验暂时不校验

    // +-----+--------+
    // | VER | METHOD |
    // +-----+--------+
    // |  1  |   1    |
    // +-----+--------+
    if stream.write(&[0x05, 0x00])? != 2 {
        return Err(Error::new(format!(
            "socks_parser: write size != 2 size: {}",
            size
        )));
    }
    let size = stream.read(&mut buffer)?;
    // +-----+-----+-------+------+----------+----------+
    // | VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |
    // +-----+-----+-------+------+----------+----------+
    // |  1  |  1  | X'00' |  1   | Variable |    2     |
    // +-----+-----+-------+------+----------+----------+
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
    let addr = get_addr(&buffer)?;
    println!("get {:?}", &buffer[..size]);
    println!("adrr {}", addr);
    Ok(stream)
}

fn get_addr(buffer: &[u8]) -> Result<SocketAddr, Error> {
    match buffer[3] {
        0x01 => {
            let mut addr = [0; 4];
            let mut port = [0; 2];
            let mut index = 0;
            for v in buffer[4..].iter() {
                match index {
                    0..=3 => addr[index] = *v,
                    4..=5 => {
                        port[index - 4] = *v;
                    }
                    _ => break,
                }
                index += 1;
            }
            Ok(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from(addr)),
                (port[0] as u16) << 8 | port[1] as u16,
            ))
        }
        0x04 => {
            let mut addr = [0; 16];
            let mut port = [0; 2];
            let mut index = 0;
            for v in buffer[4..].iter() {
                match index {
                    0..=15 => addr[index] = *v,
                    16..=17 => {
                        port[index - 4] = *v;
                    }
                    _ => break,
                }
                index += 1;
            }
            Ok(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from(addr)),
                (port[0] as u16) << 8 | port[1] as u16,
            ))
        }
        _ => Err(Error::new(format!(
            "just support v4 v6  type: {} source: {:?}",
            buffer[3], buffer
        ))),
    }
}

pub fn start_server(addr: &str) -> Result<(), Error> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
// fn read_stream(mut stream: TcpStream) -> Result<(), Error> {
//     let mut buffer = [0_u8; 5];
//     let count = stream.read(&mut buffer)?;
//     if count != 5 {
//         return Err(Error::new("read_stream: hander len != 5"));
//     }
//     let num = u8_to_u32(&buffer[..4]);
//     let data_type = buffer
//         .get(5)
//         .ok_or(Error::new("read_stream: index 5 got none"))?;
//     Ok(())
// }
