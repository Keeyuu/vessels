//use crate::core::error::Error;
////用4个u8表示一个u32
//pub fn u8_to_u32(chars: &[u8]) -> Result<u32, Error> {
//    if chars.len() != 4 {
//        return Err(Error::new("u8_to_u32: chars len != 4"));
//    }
//    use byteorder::{BigEndian, ReadBytesExt};
//    use std::io::Cursor;
//    let mut rdr = Cursor::new(chars);
//    let num = rdr.read_u32::<BigEndian>()?;
//    return Ok(num);
//}

//pub fn u32_to_u8(num: u32) -> Result<Vec<u8>, Error> {
//    use byteorder::{LittleEndian, WriteBytesExt};
//    let mut wtr: Vec<u8> = vec![];
//    wtr.write_u32::<LittleEndian>(num)?;
//    wtr.reverse();
//    Ok(wtr)
//}
