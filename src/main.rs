mod bootrom;
mod constants;
mod cpu;
mod gameboy;
mod hram;
mod lcd;
mod mem;
mod ppu;
mod wram;

use std::{env, fs::File, io::Read, process::exit};

fn file2vec(fname: &String) -> Vec<u8> {
    if let Ok(mut file) = File::open(fname) {
        let mut ret = vec![];
        file.read_to_end(&mut ret).unwrap();
        ret
    } else {
        panic!("failed to open {}.", fname);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("The file name argument is required.");
        exit(1);
    }

    let cartridge_raw = file2vec(&args[1]);
    let bootrom = bootrom::Bootrom::new(cartridge_raw.into());

    let mut gameboy = gameboy::Gameboy::new(bootrom);
    gameboy.run();
}
