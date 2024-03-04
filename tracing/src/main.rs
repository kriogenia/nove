mod trace;

use nove_core::cartridge::Rom;
use nove_core::core::NesNoveCore;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// The ROM file to read
    pub file: String,
}

fn main() {
    let args = Args::from_args();
    let content = std::fs::read(args.file).expect("failed to read rom file");
    let rom = Rom::new(&content).unwrap();

    let mut count = 0;

    let mut core = NesNoveCore::new(rom);
    core.reset();
    core.pc = 0xC000;

    println!("{}", trace::trace(&core));
    core.run(move |core| {
        count += 1;
        if count > 100 {
            std::process::exit(0);
        }
        println!("{}", trace::trace(core));
    })
    .unwrap();
}
