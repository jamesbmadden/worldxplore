mod worldgen;
mod render;
mod camera;
mod tiles;

use winit::{
  event::{Event, WindowEvent, ElementState},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use futures::executor::block_on;

// Dimensions of the world
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

// Dimensions of the camera
const CAM_WIDTH: i32 = 33;
const CAM_HEIGHT: i32 = 25;

fn main() {
  // create input manager
  let mut input = WinitInputHelper::new();
  // generate the world
  let world = worldgen::elevation_to_tiles(worldgen::generate_perlin(WIDTH, HEIGHT));
  // create a window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().with_title("WorldXPlore Alpha").build(&event_loop).unwrap();
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
  // create renderer
  let mut renderer = block_on(render::Render::new(&window, &world, CAM_WIDTH, CAM_HEIGHT));

  // run event loop
  event_loop.run(move | event, _, control_flow | {
    //*control_flow = ControlFlow::Wait;
    if input.update(&event) {

      if input.key_released(winit::event::VirtualKeyCode::Escape) || input.quit() {
        *control_flow = ControlFlow::Exit
      }

      // camera updates
      if input.key_pressed(winit::event::VirtualKeyCode::W) { camera.key_pressed(winit::event::VirtualKeyCode::W); }
      if input.key_pressed(winit::event::VirtualKeyCode::A) { camera.key_pressed(winit::event::VirtualKeyCode::A); }
      if input.key_pressed(winit::event::VirtualKeyCode::S) { camera.key_pressed(winit::event::VirtualKeyCode::S); }
      if input.key_pressed(winit::event::VirtualKeyCode::D) { camera.key_pressed(winit::event::VirtualKeyCode::D); }
      if input.key_released(winit::event::VirtualKeyCode::W) { camera.key_released(winit::event::VirtualKeyCode::W); }
      if input.key_released(winit::event::VirtualKeyCode::A) { camera.key_released(winit::event::VirtualKeyCode::A); }
      if input.key_released(winit::event::VirtualKeyCode::S) { camera.key_released(winit::event::VirtualKeyCode::S); }
      if input.key_released(winit::event::VirtualKeyCode::D) { camera.key_released(winit::event::VirtualKeyCode::D); }

      renderer.update(&world, &mut camera);
      renderer.render();

    }
  });
}