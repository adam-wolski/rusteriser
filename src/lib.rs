#![feature(test)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate tobj;
extern crate cgmath;
extern crate image;
extern crate rand;

pub mod line;
pub mod model;
pub mod color;
pub mod utils;
pub mod triangle;
pub mod gl;
pub mod shaders;

#[cfg(test)]
mod test;
