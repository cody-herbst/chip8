use std::default::Default;
use std::{thread, time};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::Thread;
use winit::keyboard::KeyCode;
use crate::graphics::display::{open};
use crate::hardware::cpu::Cpu;
use crate::hardware::memory::Memory;
use crate::Args;
use crate::graphics::grid::{Grid};

#[derive(Default, Debug)]
pub struct Registers {
    pub registers: [u8; 16],
    pub index: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16
}

pub type Keys = [bool; 16];

#[derive(Debug)]
pub enum KeyBoardEvent {
    Pressed(KeyCode),
    Released(KeyCode),
    Held(KeyCode),
}

pub fn run(args: Args) {

    let local_grid = Grid::new();
    
    // had to do this because calling lock out side of the thread blocked the mutex
    let display_height = local_grid.display_height;
    let display_width = local_grid.display_width;
    
    let grid = Arc::new(Mutex::new(local_grid));
    let cpu_grid = grid.clone();
    let display_grid = grid.clone();
    
    let keys = Arc::new(RwLock::new(Keys::default()));
    let display_keys = keys.clone();

    thread::spawn( move || {
        let file = File::open(args.rom);
        let mut buffer = Vec::new();
        file.unwrap().read_to_end(&mut buffer).unwrap();

        let mut memory = Memory::new(); // memory
        let mut registers = Registers { ..Default::default() }; // registers
        let mut cpu = Cpu::new(&mut registers, &mut memory, cpu_grid, keys);

        cpu.load_rom(&buffer);

        loop {
            let millis = time::Duration::from_millis(1);
            thread::sleep(millis);
            let instruction = cpu.fetch();
            cpu.execute(instruction);
            cpu.dec_timers();
        }
    });

    open(display_grid, display_width, display_height, display_keys).expect("TODO: panic message");

}
