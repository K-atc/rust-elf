use cstr_core::{CString, CStr};

#[macro_export]
macro_rules! try {
    ($expr:expr) => {{
        $expr?
    }}
}

#[macro_export]
macro_rules! read_u16 {
    ($elf:ident, $io:ident) => {{
        #[allow(unused_imports)]
        use io::byteorder::{BigEndian, LittleEndian, ReadBytesExt};
        match $elf.ehdr.data {
            types::ELFDATA2LSB => $io.read_u16::<LittleEndian>(),
            types::ELFDATA2MSB => $io.read_u16::<BigEndian>(),
            types::ELFDATANONE => {
                panic!("Unable to resolve file endianness");
            }
            _ => {
                panic!("Unable to resolve file endianness");
            }
        }
    }};
}

#[macro_export]
macro_rules! read_u32 {
    ($elf:ident, $io:ident) => {{
        #[allow(unused_imports)]
        use io::byteorder::{BigEndian, LittleEndian, ReadBytesExt};
        match $elf.ehdr.data {
            types::ELFDATA2LSB => $io.read_u32::<LittleEndian>(),
            types::ELFDATA2MSB => $io.read_u32::<BigEndian>(),
            types::ELFDATANONE => {
                panic!("Unable to resolve file endianness");
            }
            _ => {
                panic!("Unable to resolve file endianness");
            }
        }
    }};
}

#[macro_export]
macro_rules! read_u64 {
    ($elf:ident, $io:ident) => {{
        #[allow(unused_imports)]
        use io::byteorder::{BigEndian, LittleEndian, ReadBytesExt};
        match $elf.ehdr.data {
            types::ELFDATA2LSB => $io.read_u64::<LittleEndian>(),
            types::ELFDATA2MSB => $io.read_u64::<BigEndian>(),
            types::ELFDATANONE => {
                panic!("Unable to resolve file endianness");
            }
            _ => {
                panic!("Unable to resolve file endianness");
            }
        }
    }};
}

pub fn get_string(data: &[u8], start: usize) -> CString {
    CString::from(unsafe { CStr::from_bytes_with_nul_unchecked(&data[start..data.len()])})
}
