use std::io::{Read, Write};
use std::prelude::*;
pub mod core;
//use crate::core::data::Metadata;

use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    stream.write_all("echo".as_bytes())?;
    println!("has read some :{:?}", &buffer[..size]);
    //crate::core::num::to_num(1);
    Ok(())
}

pub fn start_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
