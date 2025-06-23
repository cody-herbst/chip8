use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use crate::graphics::grid::{Grid, GRID_X_BOXES, GRID_Y_BOXES};
use crate::hardware::memory::Memory;
use crate::system::emulator::{Keys, Registers};

/// In the future this probably isn't necessary
/// just checking out how lifetimes work.
pub struct Cpu<'a, 'b> {
    registers: &'a mut Registers,
    memory: &'b mut Memory,
    grid_lock: Arc<Mutex<Grid>>,
    keys_lock: Arc<RwLock<Keys>>,
    tick_rate: Duration,
    next_tick: Instant,
}

impl<'a, 'b> Cpu<'a, 'b> {
    pub fn new(registers : &'a mut Registers,
                       memory: &'b mut Memory,
                       grid_lock: Arc<Mutex<Grid>>,
                       keys_lock: Arc<RwLock<Keys>>,
    ) -> Cpu<'a, 'b> {
        registers.pc = 512;
        let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
        let next_tick = Instant::now() + tick_rate;
        Cpu {registers, memory, grid_lock, keys_lock, tick_rate, next_tick}
    }
    
    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let mut start = 0x200; // 512
        for byte in rom {
            self.memory.set(start, *byte);
            start += 1;
        }
        println!("ROM loaded");
    }
    
    pub fn fetch(&mut self) -> u16 {
        let first_byte = self.memory.get(self.registers.pc);
        let second_byte = self.memory.get(self.registers.pc + 1);
        
        self.registers.pc += 2;
        
        (first_byte as u16) << 8 | second_byte as u16
    }
    
    pub fn dec_timers(&mut self) {
        while Instant::now() >= self.next_tick {
            // decrement exactly one “tick” worth
            if self.registers.delay > 0 {
                self.registers.delay -= 1;
            }
            if self.registers.sound > 0 {
                self.registers.sound -= 1;
            }
            
            // schedule the *next* tick
            self.next_tick += self.tick_rate;
        }
    }
    
    pub fn execute(&mut self, instruction: u16) {
        let nib1 = (instruction & 0xF000) >> 12;
        let nib2 = (instruction & 0x0F00) >> 8;
        let nib3 = (instruction & 0x00F0) >> 4;
        let nib4 = instruction & 0x000F;
        
        match (nib1, nib2, nib3, nib4) {
            (0,0,0,0) => (),
            (0,0,0xE,0) => {
                let grid = &mut self.grid_lock.lock().unwrap();
                if instruction == 0x00E0 {
                    for x in 0..GRID_X_BOXES {
                        for y in 0..GRID_Y_BOXES {
                            grid.data[x][y] = 0;
                            grid.draw_box(x, y, 0)
                        }
                    }
                }
            },
            (0,0,0xE,0xE) => {
                if let Some(cell) = self.memory.stack.pop() { self.registers.pc = cell };
            }
            (1,_,_,_) => {
                self.registers.pc = instruction & 0xFFF;
            },
            (2,_,_,_) => {
                let address = instruction & 0xFFF;
                self.memory.stack.push(self.registers.pc);
                self.registers.pc = address;
            },
            (3,_,_,_) => {
                let reg_i = nib2 as usize;
                let value = (instruction & 0xFF) as u8;
                if self.registers.registers[reg_i] == value {
                    self.registers.pc += 2;
                }
            },
            (4,_,_,_) => {
                let reg_i = nib2 as usize;
                let value = (instruction & 0xFF) as u8;
                if self.registers.registers[reg_i] != value {
                    self.registers.pc += 2;
                }
            },
            (5,_,_,0) => {
                let reg_x = nib2 as usize;
                let reg_y = nib3 as usize;
                if self.registers.registers[reg_x] == self.registers.registers[reg_y] {
                    self.registers.pc += 2;
                }
            },
            (6,_,_,_) => {
                self.registers.registers[nib2 as usize] = (instruction & 0xFF) as u8;
            },
            (7,_,_,_) => {
                self.registers.registers[nib2 as usize] = self.registers.registers[nib2 as usize].wrapping_add((instruction & 0xFF) as u8);
            },
            (8,_,_,0) => {
                self.registers.registers[nib2 as usize] = self.registers.registers[nib3 as usize]
            },
            (8,_,_,1) => {
                self.registers.registers[nib2 as usize] |= self.registers.registers[nib3 as usize]
            },
            (8,_,_,2) => {
                self.registers.registers[nib3 as usize] &= self.registers.registers[nib2 as usize]
            }
            (8,_,_,3) => {
                self.registers.registers[nib2 as usize] ^= self.registers.registers[nib3 as usize]
            }
            (8,_,_,4) => {
                let x = nib2 as usize;
                let y = nib3 as usize;

                let (new_vx, carry) = self.registers.registers[x].overflowing_add(self.registers.registers[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.registers.registers[x] = new_vx;
                self.registers.registers[0xF] = new_vf;
            },
            (8,_,_,5) => {
                let x = nib2 as usize;
                let y = nib3 as usize;

                let (new_vx, borrow) = self.registers.registers[x].overflowing_sub(self.registers.registers[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers.registers[x] = new_vx;
                self.registers.registers[0xF] = new_vf;
            }
            (8, _, _, 6) => {
                let x = nib2 as usize;
                let lsb = self.registers.registers[x] & 1;
                self.registers.registers[x] >>= 1;
                self.registers.registers[0xF] = lsb;
            },
            (8,_,_,7) => {
                let x = nib2 as usize;
                let y = nib3 as usize;

                let (new_vx, borrow) = self.registers.registers[y].overflowing_sub(self.registers.registers[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers.registers[x] = new_vx;
                self.registers.registers[0xF] = new_vf;
            }
            (8, _, _,0xE) => {
                let x = nib2 as usize;
                let lsb = (self.registers.registers[x] >> 7) & 1;
                self.registers.registers[x] <<= 1;
                self.registers.registers[0xF] = lsb;
            }
            (9,_,_,_) => {
                let reg_x = nib2 as usize;
                let reg_y = nib3 as usize;
                if self.registers.registers[reg_x] != self.registers.registers[reg_y] {
                    self.registers.pc += 2;
                }
            },
            (0xA,_,_,_) => {
                self.registers.index = instruction & 0xFFF;
            },
            (0xB,_,_,_) => {
                self.registers.pc = (instruction & 0xFFF) + self.registers.registers[0] as u16;
            }
            (0xC,_,_,_) => {
                let random_byte: u8 = rand::random();
                self.registers.registers[nib2 as usize] = random_byte & (instruction & 0xFF) as u8;
            }
            (0xD,_,_,_) => {
                let vx = self.registers.registers[nib2 as usize];
                let vy = self.registers.registers[nib3 as usize];
                let n = nib4;
                
                let mut collision = 0;

                {
                    let grid = &mut self.grid_lock.lock().unwrap();
                    for row in 0..n {
                        let sprite_byte = self.memory.get(self.registers.index + row);
                        for bit in 0..8 {
                            if (sprite_byte & (0b1000_0000 >> bit)) != 0 {
                                // Sprites should wrap around screen, so apply modulo
                                let x = ((vx + bit) % GRID_X_BOXES as u8) as usize;
                                let y = ((vy + row as u8) % GRID_Y_BOXES as u8) as usize;

                                // Check if we're about to flip the pixel and set
                                collision |= grid.data[x][y];
                                let value = grid.data[x][y] ^ 1;
                                grid.data[x][y] = value;
                                
                                grid.draw_box(x, y, value);
                            }
                        }
                    }
                }
                
                if collision == 1{
                    self.registers.registers[0xF] = 1;
                } else {
                    self.registers.registers[0xF] = 0;
                }
            }
            (0xE,_,9,0xE) => {
                let key = self.registers.registers[nib2 as usize] as usize;
                let keys = self.keys_lock.read().unwrap(); 
                if keys[key] {
                    self.registers.pc += 2;
                }
                
            }
            (0xE,_,0xA,1) => {
                let key = self.registers.registers[nib2 as usize] as usize;
                let keys = self.keys_lock.read().unwrap();
                if !keys[key] {
                    self.registers.pc += 2;
                }
            }
            (0xF,_,0,7) => {
                self.registers.registers[nib2 as usize] = self.registers.delay;
            }
            (0xF,_,1,5) => {
                self.registers.delay = self.registers.registers[nib2 as usize]
            }
            (0xF,_,1,8) => {
                self.registers.sound = self.registers.registers[nib2 as usize]
            }
            (0xF,_,1,0xE) => {
                self.registers.index += self.registers.registers[nib2 as usize] as u16
            }
            (0xF,_,0,0xA) => {
                let x = nib2 as usize;
                let mut pressed = false;

                { // scoping the lock
                    let keys = self.keys_lock.read().unwrap();
                    for i in 0..keys.len() {
                        if keys[i] {
                            self.registers.registers[x] = i as u8;
                            pressed = true;
                            break;
                        }
                    }
                }

                if !pressed {
                    self.registers.pc -= 2;
                } else {
                    let mut released = false;
                    while !released {
                        let keys = self.keys_lock.read().unwrap();
                        if !keys[self.registers.registers[x] as usize] {
                            released = true;
                        }
                    }
                }
                
            }
            (0xF,_,2,9) => {
                self.registers.index = (self.registers.registers[nib2 as usize] * 5) as u16;
            }
            (0xF,_,3,3) => {
                let vx = self.registers.registers[nib2 as usize];
                let x = vx / 100; 
                let y = (vx / 10) % 10;
                let z = vx % 10;
                self.memory.set(self.registers.index, x);
                self.memory.set(self.registers.index + 1, y);
                self.memory.set(self.registers.index + 2, z);
            }
            (0xF,_,5,5) => {
                let x = nib2 as usize;
                let i = self.registers.index as usize;
                for idx in 0..=x {
                    self.memory.set((i + idx) as u16, self.registers.registers[idx]);
                }
            }
            (0xF,_,6,5) => {
                let x = nib2 as usize;
                let i = self.registers.index as usize;
                for idx in 0..=x {
                    self.registers.registers[idx] = self.memory.get((i + idx) as u16);
                }
            }
            _ => { println!("Unknown instruction opcode!!!! {} {} {} {}", nib1, nib2, nib3, nib4 ); }
        }
    }
}

