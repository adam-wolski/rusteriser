#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

pub mod window;


const WINDOW_WIDTH: u32 = 16;
const WINDOW_HEIGHT: u32 = 16;

fn main() {
    env_logger::init().unwrap();
    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut backbuffer: Vec<u8> = Vec::with_capacity((WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize);

    for _ in (0..backbuffer.capacity()).filter(|i| i % 4 == 0) {
        backbuffer.push(0); // B
        backbuffer.push(0); // G
        backbuffer.push(255); // R
        backbuffer.push(0); // A
    }

    while window.is_running() {
        window.backbuffer_fill(backbuffer.as_ref());
        window.swap();
    }
}
