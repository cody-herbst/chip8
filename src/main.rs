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
}


fn main() {
    let args = Args::parse();
    run(args);
}

