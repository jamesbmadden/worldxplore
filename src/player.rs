use std::collections::HashSet;
use bytemuck::{Pod, Zeroable};
use crate::tiles;
use crate::render; // for the tileset size constants

use std::f32::consts::PI;

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
    self.uniforms = Uniforms { 
      translate_vector: [ x_offset * tile_width, y_offset * tile_height ], 
      is_swimming: self.is_swimming.into(),
      time: self.uniforms.time + 0.01,
      light_intensity: light_intensity(self.uniforms.time + 0.01)
    };

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
  pub translate_vector: [f32; 2],
  pub is_swimming: i32,
  pub time: f32,
  pub light_intensity: [f32; 3]
}

impl Uniforms {
  pub fn default() -> Self {
    Uniforms {
      translate_vector: [0., 0.],
      is_swimming: 0,
      time: 0.,
      light_intensity: light_intensity(0.)
    }
  }
}

// create a light intensity for the shader based on current time
pub fn light_intensity (time: f32) -> [f32; 3] {

  let day_length: f32 = 36.; // length of the day (currently a minute)

  let offset_time = 2. * PI / day_length * time; // so that the cycle happens every length of day instead of pi

  let r: f32 = 1_f32.min(0.03_f32.max(offset_time.sin() + 1.)); // r value; sliced sine
  let g: f32 = 1_f32.min(0.06_f32.max(offset_time.sin() + 0.9)); // g value; sliced sine, make it rise slower for sunrise / sunset
  let b: f32 = 1_f32.min(0.1_f32.max(offset_time.sin() + 0.8)); // b value; sliced sine, make it rise slowest for sunrise / sunset

  return [r, g, b];
}

// return the vertices and indices to form player sprite
pub fn player_vertices (width: i32, height: i32) -> (Vec<render::Vertex>, Vec<u16>) {

  let tile_width: f32 = 1. / width as f32;
  let tile_height: f32 = 1. / height as f32;
  let texture_width: f32 = 8. / render::TILESET_WIDTH as f32;
  let texture_height: f32 = 8. / render::TILESET_HEIGHT as f32;
  // player data:
  ( 
    vec![ // player vertices - texture coords depend on whether in water or not for different texture
      render::Vertex { pos: [ -tile_width, tile_height * 3. ], tex_coords: [ 0., texture_height * 4. ] }, // top left
      render::Vertex { pos: [ -tile_width, tile_height * -3. ], tex_coords: [ 0., texture_height * 7. ] }, // bottom left
      render::Vertex { pos: [ tile_width, tile_height * -3. ], tex_coords: [ texture_width, texture_height * 7. ] }, // bottom right
      render::Vertex { pos: [ tile_width, tile_height * 3. ], tex_coords: [ texture_width, texture_height * 4. ] } // top right
    ],
    vec![ 0, 1, 2, 0, 2, 3 ] // player indices
  )
}