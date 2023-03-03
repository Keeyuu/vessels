use crate::core::error::Error;
// use crate::core::num::*;
// use std::collections::HashMap;
// use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
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
    match buffer[3] {
        0x01 => {
            let x = Ipv4Addr::from([0;4]);
        }
        _ => {}
    }
    println!("get {:?}", &buffer[..size]);
    Ok(stream)
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
