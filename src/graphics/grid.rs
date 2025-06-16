pub const GRID_X_BOXES: i16 = 64;
pub const GRID_Y_BOXES: i16 = 32;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct Grid {
    box_width_x: i16,
    box_height_y: i16,
    pub display_height: u32,
    pub display_width: u32,
    pub data:[[u8; GRID_Y_BOXES as usize]; GRID_X_BOXES as usize],
}

impl Grid {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        let box_width_x = 16;
        let box_height_y = 16;
        let display_width = GRID_X_BOXES * box_width_x;
        let display_height = GRID_Y_BOXES * box_height_y;
        let grid = [[0; 32]; 64];

        Self {
            box_height_y,
            box_width_x,
            data: grid,
            display_width: display_width as u32,
            display_height: display_height as u32,
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub(crate) fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.display_width as usize) as i16;
            let y = (i / self.display_width as usize) as i16;

            let box_x = x / self.box_width_x;
            let box_y = y / self.box_height_y;

            let mut rgba = [239, 239, 240, 1];
            if box_x < GRID_X_BOXES && box_y < GRID_Y_BOXES {
                rgba = if self.data[box_x as usize][box_y as usize] == 1 {
                    [0xff, 0xff, 0xff, 0xff]
                } else {
                    [0x00, 0x00, 0x00, 0x01]
                };
            }

            pixel.copy_from_slice(&rgba);
        }
    }
}