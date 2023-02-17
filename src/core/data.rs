//|4个u8| 数据包总长度 |1个u8| 数据类型 |----| 真实数据 |一个u8| 结尾符
use crate::core::error::Error;

pub fn is_end(char: u8) -> bool {
    match char {
        b'c' => false,
        b'e' => true,
        _ => true,
    }
}

pub enum Metadata<'a> {
    Transmit(&'a [u8]),  //传输数据
    Handshake(&'a [u8]), //握手包
    Heartbeat(), //心跳包
}

impl<'a> Metadata<'a> {
    pub fn new(data_type: u8, data: &'a [u8]) -> Result<Metadata<'a>, Error> {
        match data_type {
            1 => Ok(Metadata::Transmit(data)),
            2 => Ok(Metadata::Handshake(data)),
            3 => Ok(Metadata::Heartbeat()),
            _ => Err(Error::new("data_type unknow")),
        }
    }
}
