mod worldgen;
mod render;
mod player;
mod tiles;
mod ui;

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
  #[cfg(feature = "web-sys")]
  onsole_error_panic_hook::set_once();
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
  // create player state
  let mut player = player::Player::new(CAM_WIDTH, CAM_HEIGHT);
  // create renderer
  let mut renderer = block_on(render::Render::new(&window, &world, &player, CAM_WIDTH, CAM_HEIGHT));

  // run event loop
  event_loop.run(move | event, _, control_flow | {
    //*control_flow = ControlFlow::Wait;
    if input.update(&event) {

      if input.key_released(winit::event::VirtualKeyCode::Escape) {
        player.paused = !player.paused;
      }
      if input.quit() {
        *control_flow = ControlFlow::Exit;
      }

      if input.key_pressed(winit::event::VirtualKeyCode::W) { player.key_pressed(winit::event::VirtualKeyCode::W); }
      if input.key_pressed(winit::event::VirtualKeyCode::A) { player.key_pressed(winit::event::VirtualKeyCode::A); }
      if input.key_pressed(winit::event::VirtualKeyCode::S) { player.key_pressed(winit::event::VirtualKeyCode::S); }
      if input.key_pressed(winit::event::VirtualKeyCode::D) { player.key_pressed(winit::event::VirtualKeyCode::D); }
      if input.key_released(winit::event::VirtualKeyCode::W) { player.key_released(winit::event::VirtualKeyCode::W); }
      if input.key_released(winit::event::VirtualKeyCode::A) { player.key_released(winit::event::VirtualKeyCode::A); }
      if input.key_released(winit::event::VirtualKeyCode::S) { player.key_released(winit::event::VirtualKeyCode::S); }
      if input.key_released(winit::event::VirtualKeyCode::D) { player.key_released(winit::event::VirtualKeyCode::D); }

      renderer.update(&world, &mut player);
      renderer.render();

    }
  });
}