use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use crossbeam_channel::Sender;
use error_iter::ErrorIter;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::*;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::graphics::grid::{Grid};
use crate::system::emulator::{KeyBoardEvent, Keys};
use crate::system::emulator::KeyBoardEvent::{Held, Pressed, Released};

const KEYS : [KeyCode; 16] = [
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyZ,
    KeyX,
    KeyC,
    KeyV
];

pub fn open( grid_lock : Arc<Mutex<Grid>>, display_width: usize, display_height: usize, keys : Arc<RwLock<Keys>>) -> Result<(), Error> {
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
            
            // send key events
            for (i,key) in KEYS.iter().enumerate() {
                key_event_check(&input, key, keys.clone(), i);
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

fn key_event_check(input : &WinitInputHelper, key_code : &KeyCode, keys : Arc<RwLock<Keys>>, i: usize) {
    if input.key_pressed(*key_code) {
        let keys_unlock = &mut keys.write().unwrap();
        keys_unlock[i] = true;
    } else if input.key_released(*key_code) {
        let keys_unlock = &mut keys.write().unwrap();
        keys_unlock[i] = false;
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

