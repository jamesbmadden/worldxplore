mod worldgen;
mod render;
mod camera;

use winit::{
  event::{Event, WindowEvent, ElementState},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use futures::executor::block_on;

// Dimensions of the world
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

// Dimensions of the camera
const CAM_WIDTH: i32 = 32;
const CAM_HEIGHT: i32 = 24;

fn main() {
  // generate the world
  let world = worldgen::elevation_to_tiles(worldgen::generate_perlin(WIDTH, HEIGHT));
  // create a window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  // set window options
  window.set_title("WorldXPlore Alpha");
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
  // create camera
  let mut camera = camera::Camera::new(CAM_WIDTH, CAM_HEIGHT);
  // send to rendere
  let mut renderer = block_on(render::Render::new(&window, world, CAM_WIDTH, CAM_HEIGHT));

  // run event loop
  event_loop.run(move | event, _, control_flow | {
    *control_flow = ControlFlow::Wait;

    match event {
      
      Event::WindowEvent {
        ref event,
        ..
      } => match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
  
        WindowEvent::KeyboardInput { input, .. } => {
          if input.state == ElementState::Pressed {
            camera.key_pressed(input.virtual_keycode.unwrap());
          } else if input.state == ElementState::Released {
            camera.key_released(input.virtual_keycode.unwrap());
          }
        },
        _ => ()
      },

      Event::RedrawRequested (_) => {
        renderer.render();
      },
      _ => ()
    }
  });
}