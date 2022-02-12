extern crate clap;
extern crate elf;

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

    let elf = elf::File::open_path(matches.value_of("ELF_FILE").unwrap()).unwrap();

    let symbols = match elf.get_section(".symtab")
    {
        Some(text) => elf.get_symbols(text).unwrap(),
        None => Vec::new(),
    };

    for symbol in symbols.iter() {
        println!("{:#}", symbol);
    }
}