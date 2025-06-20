use crate::graphics::display;

pub const GRID_X_BOXES: usize = 64;
pub const GRID_Y_BOXES: usize = 32;

pub const BOX_WIDTH_X : usize = 16;
pub const BOX_HEIGHT_Y: usize = 16;

pub const DISPLAY_LEN: usize = GRID_X_BOXES * BOX_WIDTH_X * GRID_Y_BOXES * BOX_HEIGHT_Y * 4/* for pixel*/;

type COLOR = [u8; 4];

const BLACK : COLOR = [0x00, 0x00, 0x00, 0x01];
const WHITE : COLOR = [0xff, 0xff, 0xff, 0xff];

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct Grid {
    pub display_height: usize,
    pub display_width: usize,
    pub data:[[u8; GRID_Y_BOXES]; GRID_X_BOXES],
    pub display : Vec<u8>,
}

impl Grid {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        
        let display_width = GRID_X_BOXES * BOX_WIDTH_X;
        let display_height = GRID_Y_BOXES * BOX_HEIGHT_Y;
        let grid = [[0; 32]; 64];
        
        let display = vec![0; DISPLAY_LEN];

        Self {
            data: grid,
            display,
            display_width,
            display_height,
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub(crate) fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            
            pixel.copy_from_slice(&self.display[i*4..i*4 + 4]);
        }
    }
    
    pub fn render_box(&mut self, x: usize, y : usize, v : u8) {
        // x = 1, y = 0
        // [16..32], 
        
        // our starting index
        let mut i: usize = (x * BOX_WIDTH_X * 4) + (y * BOX_HEIGHT_Y * GRID_X_BOXES * BOX_WIDTH_X * 4);
        let upper_limit = BOX_WIDTH_X * 4;
        
        for row in 0..16 {
            let upper_bound = upper_limit + i;
            for pixel in self.display[i..upper_bound].chunks_exact_mut(4) {
                if v == 1 {
                    pixel.copy_from_slice(WHITE.as_slice()); 
                } else {
                    pixel.copy_from_slice(BLACK.as_slice());
                }
            }
            i += self.display_width * 4; // this should move us to the next pixel row
        }
        
        // for (i, pixel) in self.display.chunks_exact_mut(4).enumerate() { 
        //     if (((i as u32 % self.display_width) as i16) / (BOX_WIDTH_X as i16)) as u8 == (x as u8) && 
        //         (((i as u32 / self.display_height) as i16) / (BOX_HEIGHT_Y as i16)) as u8 == (y as u8) {
        //         if v == 1 {
        //             pixel.copy_from_slice(WHITE.as_slice());
        //         } else {
        //             pixel.copy_from_slice(BLACK.as_slice());
        //         }
        //     };
        // }
    }
}