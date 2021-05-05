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

// Dimensions of the world
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

// Dimensions of the camera
const CAM_WIDTH: i32 = 33;
const CAM_HEIGHT: i32 = 25;

async fn run() {
  #[cfg(target_arch = "wasm32")]
  console_error_panic_hook::set_once();
  // create input manager
  let mut input = WinitInputHelper::new();
  // generate the world
  let seed = rand::random::<u32>();
  let world = worldgen::elevation_to_tiles(worldgen::generate_perlin(WIDTH, HEIGHT, seed));
  // create a window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().with_title("WorldXPlore Alpha").build(&event_loop).unwrap();
  // create a canvas if we're running in web
  #[cfg(target_arch = "wasm32")]
  {
    use winit::platform::web::WindowExtWebSys;

    web_sys::window()
      .and_then(|win| win.document())
      .and_then(|doc| doc.body())
      .and_then(|body| {
        body.append_child(&web_sys::Element::from(window.canvas()))
          .ok()
      })
      .expect("couldn't append canvas to document body");
  }
  // create player state
  let mut player = player::Player::new(CAM_WIDTH, CAM_HEIGHT, seed);
  // create renderer
  let mut renderer = render::Render::new(&window, &world, &mut player, CAM_WIDTH, CAM_HEIGHT).await;

  // run event loop
  event_loop.run(move | event, _, control_flow | {
    //*control_flow = ControlFlow::Wait;
    match event {
      Event::WindowEvent { ref event, window_id } => match event {
        WindowEvent::Resized(physical_size) => {
          renderer.resize(*physical_size);
        },
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          renderer.resize(**new_inner_size);
        },
        _ => ()
      }
      _ => ()
    }
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

      let (mouse_x, mouse_y) = input.mouse().unwrap_or((0., 0.));

      // adjust mouse position to the same coordinate system as WGPU
      let relative_mouse_pos: [f32; 2] = [(mouse_x / window.inner_size().width as f32 - 0.5) * 2., (1. - mouse_y / window.inner_size().height as f32 - 0.5) * 2. ]; 

      renderer.update(&world, &mut player, relative_mouse_pos, input.mouse_pressed(0), control_flow);
      renderer.render();

    }
  });
}

fn main() {
  #[cfg(not(target_arch = "wasm32"))]
  futures::executor::block_on(run());
  #[cfg(target_arch = "wasm32")]
  wasm_bindgen_futures::spawn_local(run());
}