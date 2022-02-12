extern crate clap;
extern crate elf;
extern crate cstr_core;

use std::fs::File;
use cstr_core::CString;
use elf::ElfFile;
use clap::{Arg, App};

fn main() {
    let matches = App::new("List symbols of given ELF file")
        .version("1.0")
        .author("Tomori Nao (@K_atc)")
        .about("Does awesome things")
        .arg(Arg::new("ELF_FILE")
            .help("The elf file to parse")
            .required(true)
            .index(1))
        .get_matches();

    let mut elf_file = File::open(matches.value_of("ELF_FILE").unwrap()).unwrap();
    let elf = ElfFile::parse(&mut elf_file).unwrap();

    println!("{:#?}", elf.get_section(&CString::new(".symtab").unwrap()));
    let symbols = elf.get_symbols(elf.get_section(&CString::new(".symtab").unwrap()).unwrap()).unwrap();

    for symbol in symbols.iter() {
        println!("{:#}", symbol);
    }
}