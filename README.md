# Chip8 Emulator

A Chip8 emulator written in Rust. This project implements a fully functional Chip8 virtual machine that can run classic Chip8 ROMs.

## Features

- Complete implementation of the Chip8 instruction set
- Real-time emulation with configurable speed
- Graphical display using the Pixels library
- Keyboard input mapping for game controls
- Sound and timer support
- ROM loading from file

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

### Building from Source

1. Clone the repository
2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/`.

## Usage

Run the emulator with a ROM file:

```bash
chip8 --rom path/to/your/rom
```

You can also configure the emulation speed by specifying the delay between cycles:

```bash
chip8 --rom path/to/your/rom --delay 5
```

The delay is specified in milliseconds and defaults to 3ms if not provided.

## Controls

The Chip8 uses a 16-key hexadecimal keypad. This emulator maps those keys to the following keyboard layout:

```
Original:    Mapped to:
1 2 3 C      1 2 3 4
4 5 6 D      Q W E R
7 8 9 E      A S D F
A 0 B F      Z X C V
```

## Project Structure

- `src/graphics/` - Display and rendering components
- `src/hardware/` - CPU and memory implementation
- `src/system/` - Main emulator logic and coordination

## Dependencies

- `pixels` - GPU-accelerated pixel frame rendering
- `winit` - Cross-platform window handling
- `clap` - Command-line argument parsing
- `rand` - Random number generation
- `crossbeam-channel` - Multi-producer, multi-consumer channels for message passing

## License

This project is open source and available under the MIT License.

## Acknowledgements

This project is inspired by the original Chip8 interpreter from the 1970s and various resources on Chip8 emulation.
