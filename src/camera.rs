use std::collections::HashSet;
use bytemuck::{Pod, Zeroable};

pub struct Camera {
  pub keys_down: HashSet<winit::event::VirtualKeyCode>,
  pub width: i32,
  pub height: i32,
  pub x: f32,
  pub y: f32,
  pub x_speed: f32,
  pub y_speed: f32,
  pub uniforms: Uniforms
}

impl Camera {

  pub fn new (width: i32, height: i32) -> Self {
    Camera {
      keys_down: HashSet::new(),
      x: 0., y: 0., x_speed: 0., y_speed: 0.,
      width, height,
      uniforms: Uniforms::default()
    }
  }

  // if keys are pressed, update x and y values
  pub fn update (&mut self) {
    // movement speed stuff
    self.x_speed /= 1.2;
    self.y_speed /= 1.2;

    if self.keys_down.contains(&winit::event::VirtualKeyCode::A) {
      self.x_speed -= 0.02;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::D) {
      self.x_speed += 0.02;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::S) {
      self.y_speed += 0.02;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::W) {
      self.y_speed -= 0.02;
    }

    self.x += self.x_speed;
    self.y += self.y_speed;

    // prevent from raising or lowering the x or y past bounds
    if self.x < 0. {
      self.x = 0.;
    }
    if self.y < 0. {
      self.y = 0.;
    }

    // make transforms for how much to offset tiles for smoother scrolling
    let x_offset = self.x as f32 % 1.;
    let y_offset = self.y as f32 % 1.;
    // coordinate space is -1.0 to 1.0, so size should be double as big as 1 / size
    let tile_width = 2. / self.width as f32;
    let tile_height = 2. / self.height as f32;
    // set as uniforms to be rendered with
    self.uniforms = Uniforms { translate_vector: [ x_offset * tile_width, y_offset * tile_height ] };

  }

  // key pressed, add it to keys down
  pub fn key_pressed (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.insert(key);
  }
  // key released, add it to keys down
  pub fn key_released (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.remove(&key);
  }

}


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Uniforms {
  pub translate_vector: [f32; 2]
}

impl Uniforms {
  pub fn default() -> Self {
    Uniforms {
      translate_vector: [0., 0.]
    }
  }
}