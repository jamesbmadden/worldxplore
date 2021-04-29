mod worldgen;
mod render;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use futures::executor::block_on;

const WIDTH: i32 = 11;
const HEIGHT: i32 = 11;

fn main() {
  // generate the world
  let world = worldgen::elevation_to_tiles(worldgen::generate_perlin(WIDTH, HEIGHT));
  // create a window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  // create a canvas if we're running in web
  #[cfg(feature = "web-sys")]
  {
    use winit::platform::web::WindowExtWebSys;

    let canvas = window.canvas();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.append_child(&canvas)
      .expect("Append canvas to HTML body");
  }
  // send to renderer
  let renderer = block_on(render::Render::new(&window, world));

  // run event loop
  event_loop.run(move | event, _, control_flow | {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::RedrawRequested (_) => {
        renderer.render();
      },
      _ => ()
    }
  });
}