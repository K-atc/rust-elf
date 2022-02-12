#![no_std]

pub(crate) mod utils;
pub mod types;
pub mod elf_file;

// Export
pub use elf_file::ElfFile;
pub use elf_file::Section;

extern crate alloc;
extern crate cstr_core;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
extern crate byteorder;

#[cfg(not(feature = "std"))]
extern crate acid_io;
#[cfg(not(feature = "std"))]
use acid_io as io;
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use io::byteorder;
