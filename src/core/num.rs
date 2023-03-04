use crate::core::error::Error;
//用8个u8表示一个u64
pub fn u8_to_u64(chars: &[u8]) -> Result<u64, Error> {
    if chars.len() != 8 {
        return Err(Error::new(format!(
            "u8_to_u64: chars len != 8 len: {}",
            chars.len()
        )));
    }
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::Cursor;
    let mut rdr = Cursor::new(chars);
    let num = rdr.read_u64::<BigEndian>()?;
    return Ok(num);
}

pub fn u64_to_u8(num: u64) -> Result<Vec<u8>, Error> {
    use byteorder::{LittleEndian, WriteBytesExt};
    let mut wtr: Vec<u8> = vec![];
    wtr.write_u64::<LittleEndian>(num)?;
    wtr.reverse();
    Ok(wtr)
}
