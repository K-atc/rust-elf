use cstr_core::{CString, CStr};

use super::io;
#[allow(unused_imports)]
use io::byteorder::ByteOrder;
#[allow(unused_imports)]
use io::byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::types;
use crate::types::Data;
use core::result::Result;

pub(crate) fn read_u16<T: io::Read>(data: &Data, cursor: &mut T) -> Result<u16, io::Error> {
    let mut buf = [0; 2];
    cursor.read(&mut buf)?;
    match *data {
        types::ELFDATA2LSB => Ok(LittleEndian::read_u16(&buf)),
        types::ELFDATA2MSB => Ok(BigEndian::read_u16(&buf)),
        types::ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
        }
    }
}

pub(crate) fn read_u32<T: io::Read>(data: &Data, cursor: &mut T) -> Result<u32, io::Error> {
    let mut buf = [0; 4];
    cursor.read(&mut buf)?;
    match *data {
        types::ELFDATA2LSB => Ok(LittleEndian::read_u32(&buf)),
        types::ELFDATA2MSB => Ok(BigEndian::read_u32(&buf)),
        types::ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
        }
    }
}

pub(crate) fn read_u64<T: io::Read>(data: &Data, cursor: &mut T) -> Result<u64, io::Error> {
    let mut buf = [0; 8];
    cursor.read(&mut buf)?;
    match *data {
        types::ELFDATA2LSB => Ok(LittleEndian::read_u64(&buf)),
        types::ELFDATA2MSB => Ok(BigEndian::read_u64(&buf)),
        types::ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
        }
    }
}

pub(crate) fn get_string(data: &[u8], start: usize) -> CString {
    CString::from(unsafe { CStr::from_bytes_with_nul_unchecked(&data[start..data.len()])})
}
