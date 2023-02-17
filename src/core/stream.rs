use crate::core::error::Error;
use crate::core::num::*;
use std::io::prelude::*;
use std::net::TcpStream;

fn read_stream(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0_u8; 5];
    let count = stream.read(&mut buffer)?;
    if count != 5 {
        return Err(Error::new("read_stream: hander len != 5"));
    }
    let num = u8_to_u32(&buffer[..4]);
    let data_type = buffer
        .get(5)
        .ok_or(Error::new("read_stream: index 5 got none"))?;
    Ok(())
}
