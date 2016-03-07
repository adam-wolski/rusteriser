#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate tobj;

pub mod window;
pub mod line;
pub mod backbuffer;
pub mod model;

// Here be globals
const WINDOW_WIDTH: u32 = 12;
const WINDOW_HEIGHT: u32 = 12;
// -------------------------


/// Simple color structure.
#[derive(Clone, Copy)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


fn main() {
    env_logger::init().unwrap();

    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut backbuffer = backbuffer::Backbuffer::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);

    for point in line::Line::new(0, 0, 10, 10) {
        debug!("{:?}", point);
        backbuffer.set_loc((point.0 as usize, point.1 as usize), (255, 255, 255, 255));
    }

    debug!("{:#?}", backbuffer);

    while window.is_running() {
        window.backbuffer_fill(backbuffer.data_as_ref());
        window.swap();
    }
}
