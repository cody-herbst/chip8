mod graphics;
mod hardware;
mod system;

use clap::{Parser};
use crate::system::emulator::run;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Chip8 binary
    #[arg(short, long)]
    rom: String,

    /// Emulation loop delay in milliseconds
    #[arg(short, long, default_value_t = 3)]
    delay: u64,
}


fn main() {
    let args = Args::parse();
    run(args);
}
