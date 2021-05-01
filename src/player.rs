use std::collections::HashSet;
use bytemuck::{Pod, Zeroable};
use crate::tiles;

pub struct Player {
  pub keys_down: HashSet<winit::event::VirtualKeyCode>,
  pub width: i32,
  pub height: i32,
  pub x: f32,
  pub y: f32,
  pub x_speed: f32,
  pub y_speed: f32,
  pub uniforms: Uniforms,
  pub is_swimming: bool
}

impl Player {

  pub fn new (width: i32, height: i32) -> Self {
    Player {
      keys_down: HashSet::new(),
      x: 0., y: 0., x_speed: 0., y_speed: 0.,
      width, height,
      uniforms: Uniforms::default(),
      is_swimming: false
    }
  }

  // if keys are pressed, update x and y values
  pub fn update (&mut self, world: &Vec<Vec<tiles::TileProperties>>) {
    // movement speed stuff
    if self.is_swimming {
      self.x_speed /= 1.8;
      self.y_speed /= 1.8;
    } else {
      self.x_speed /= 1.2;
      self.y_speed /= 1.2;
    }

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
    // attempt to move
    self.x += self.x_speed;
    // check if you can actually move to the position
    if world[(self.x + 0.5).floor() as usize + (self.width / 2) as usize][self.y.floor() as usize + (self.height / 2) as usize + 2].solid {
      self.x -= self.x_speed;
    }
    // same thing as above
    self.y += self.y_speed;
    if world[(self.x + 0.5).floor() as usize + (self.width / 2) as usize][self.y.floor() as usize + (self.height / 2) as usize + 2].solid {
      self.y -= self.y_speed;
    }

    // set in water to false before checking if we are in water
    self.is_swimming = false;
    // if move lands us in water, set in water to true
    // adjustments are made to set the comparison point at the centre of the character's feet instead of
    // the top left of the screen
    // values <= 1 are water varities
    if world[(self.x + 0.5).floor() as usize + (self.width / 2) as usize][self.y.floor() as usize + (self.height / 2) as usize + 2].swimmable {
      self.is_swimming = true;
    }

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