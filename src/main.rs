mod worldgen;
mod render;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

const WIDTH = 10;
const HEIGHT = 10;

fn main() {
  // generate the world
  let world = worldgen::elevation_to_tiles(worldgen::generate_perlin(11, 11));
  // create a window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  // send to renderer
  let renderer = render::Render::new();
}