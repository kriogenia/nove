extern crate env_logger;
extern crate nove_core;

use std::env;

use nove_core::cartridge::Rom;
use nove_core::core::NesNoveCore;
use nove_core::interrupt::InterruptFlag;

fn main() {
    let mut args = env::args().skip(1);
    let rom = args.next().expect("rom file path");

    env_logger::builder()
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();

    let content = std::fs::read(rom).expect("failed to read rom file");
    let rom = Rom::new(&content).unwrap();

    let mut core = NesNoveCore::new(rom);
    core.reset();
    core.pc = 0xC000;

    loop {
        if let InterruptFlag::BRK = core.tick().unwrap() {
            return;
        }
    }
}
