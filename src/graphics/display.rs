use std::sync::{Arc, Mutex};
use error_iter::ErrorIter;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::graphics::grid::{Grid};

pub struct Display {
    event_loop: EventLoop<()>,
    input_helper: WinitInputHelper,

}

pub fn open( grid_lock : Arc<Mutex<Grid>>, display_width: usize, display_height: usize) -> Result<(), Error> {
    env_logger::init();


    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(display_width as f64, display_height as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(display_width as u32, display_height as u32, surface_texture)?
    };

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            {
                let grid = grid_lock.lock().unwrap();
                pixels.frame_mut().copy_from_slice(&grid.display[..])
            }
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {

            if input.key_pressed(KeyCode::KeyA) {
                println!("Key A pressed");
            } else if input.key_held(KeyCode::KeyA) {
                println!("Key A held");
            } else if input.key_released(KeyCode::KeyA) {
                println!("Key A Released");
            }

            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}



fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

