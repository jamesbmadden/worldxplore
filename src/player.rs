use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use bytemuck::{Pod, Zeroable};
use serde::{Serialize, Deserialize};
use crate::tiles;
use crate::render; // for the tileset size constants
use crate::ui;
use crate::worldgen;

use std::f32::consts::PI;

pub struct Player<'a> {
  pub keys_down: HashSet<winit::event::VirtualKeyCode>,
  pub width: i32,
  pub height: i32,
  pub x: f32,
  pub y: f32,
  pub x_speed: f32,
  pub y_speed: f32,
  pub uniforms: Uniforms,
  pub is_swimming: bool,
  pub health: f32,
  pub max_health: f32,
  pub paused: bool,
  pub pause_type: PauseType,
  pub seed: u32,
  pub world_name: String,
  pub inventory: Vec<tiles::ItemProperties<'a>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
  pub x: f32,
  pub y: f32,
  pub health: f32,
  pub max_health: f32,
  pub seed: u32,
  pub time: f32
}

/**
 * The various ways the game can be paused.
 */
pub enum PauseType {
  Pause,
  Inventory
}

impl Player<'_> {

  pub fn new (width: i32, height: i32, seed: u32) -> Self {
    Player {
      keys_down: HashSet::new(),
      x: 0., y: 0., x_speed: 0., y_speed: 0.,
      width, height, seed,
      uniforms: Uniforms::default(),
      is_swimming: false,
      health: 6., max_health: 6.,
      paused: false,
      pause_type: PauseType::Pause,
      world_name: String::from("New Game"),
      inventory: vec![tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK, tiles::STICK]
    }
  }

  // if keys are pressed, update x and y values
  pub fn update (&mut self, world: &mut Vec<Vec<tiles::TileProperties>>, cam_width: i32, cam_height: i32) {

    self.width = cam_width;
    self.height = cam_height;
    
    // only update the player position if the game is paused
    if !self.paused {
      self.move_character(world);
      self.uniforms.time += 0.01;
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
      time: self.uniforms.time,
      light_intensity: light_intensity(self.uniforms.time)
    };

  }

  pub fn move_character (&mut self, world: &Vec<Vec<tiles::TileProperties>>) {
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
  }

  // key pressed, add it to keys down
  pub fn key_pressed (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.insert(key);
  }
  // key released, add it to keys down
  pub fn key_released (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.remove(&key);
  }

  pub fn gen_ui_vertices (&mut self, mouse_pos: [f32; 2], mouse_down: bool, control_flow: &mut winit::event_loop::ControlFlow, mut world: &mut Vec<Vec<tiles::TileProperties>>) -> (Vec<render::Vertex>, Vec<u16>) {

    let mut vertices: Vec<render::Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    // create variables for texture locations
    let tile_width: f32 = 4. / self.width as f32;
    let tile_height: f32 = 4. / self.height as f32;
    let texture_width: f32 = 8. / render::TILESET_WIDTH as f32;
    let texture_height: f32 = 8. / render::TILESET_HEIGHT as f32;

    // generate health vertices
    for heart in 0..((self.max_health / 2.).ceil() as i32) {
      // get positions for heart
      let heart_x: f32 = -1.0 + tile_width * heart as f32 + (tile_width / 10.);
      let heart_y: f32 = 1.0 - tile_height / 10.;
      // add the vertices
      vertices.push(render::Vertex { pos: [ heart_x, heart_y ], tex_coords: [ texture_width * 2., texture_height * 4. ], animation_frames: 1.}); // top left
      vertices.push(render::Vertex { pos: [ heart_x, heart_y - tile_height * 9. / 10. ], tex_coords: [ texture_width * 2., texture_height * 5. ], animation_frames: 1.}); // bottom left
      vertices.push(render::Vertex { pos: [ heart_x + tile_width * 9. / 10., heart_y - tile_height * 9. / 10. ], tex_coords: [ texture_width * 3., texture_height * 5. ], animation_frames: 1.}); // bottom right
      vertices.push(render::Vertex { pos: [ heart_x + tile_width * 9. / 10., heart_y ], tex_coords: [ texture_width * 3., texture_height * 4. ], animation_frames: 1.}); // top right
      // add in the indices
      let len = vertices.len();
      indices.append(&mut vec![ (len - 4).try_into().unwrap(), (len - 3).try_into().unwrap(), (len - 2).try_into().unwrap(), (len - 4).try_into().unwrap(), (len - 2).try_into().unwrap(), (len - 1).try_into().unwrap() ]);
    }

    // if game is paused, add text and buttons
    if self.paused {

      // black background
      vertices.push(render::Vertex { pos: [ -1., 1. ], tex_coords: [ texture_width * 5., texture_height * 4. ], animation_frames: 1.}); // top left
      vertices.push(render::Vertex { pos: [ -1., -1. ], tex_coords: [ texture_width * 5., texture_height * 5. ], animation_frames: 1.}); // bottom left
      vertices.push(render::Vertex { pos: [ 1., -1. ], tex_coords: [ texture_width * 6., texture_height * 5. ], animation_frames: 1.}); // bottom right
      vertices.push(render::Vertex { pos: [ 1., 1. ], tex_coords: [ texture_width * 6., texture_height * 4. ], animation_frames: 1.}); // top right
      // add in the indices
      let len = vertices.len();
      indices.append(&mut vec![ (len - 4).try_into().unwrap(), (len - 3).try_into().unwrap(), (len - 2).try_into().unwrap(), (len - 4).try_into().unwrap(), (len - 2).try_into().unwrap(), (len - 1).try_into().unwrap() ]);

      let mut pause_ui_vertices: Vec<render::Vertex> = Vec::new();

      pause_ui_vertices = match self.pause_type {
        PauseType::Pause => {

          // The pause title
          ui::Group {
            children: vec![
              ui::Label { pos: [0., 0.5], text: String::from("Paused"), size_x: tile_width, size_y: tile_height }.gen_vertices(),
              ui::Button { pos: [0., 0.], label: String::from("Resume"), click: || { self.paused = false; } }.gen_vertices(&mouse_pos, mouse_down),
              ui::Button { pos: [0., -0.2], label: String::from("Save Game"), click: || { self.write_out_gamedata(); } }.gen_vertices(&mouse_pos, mouse_down),
              ui::Button { pos: [0., -0.4], label: String::from("Load Game"), click: || { self.load_gamedata( self.read_gamedata()); self.paused = false; } }.gen_vertices(&mouse_pos, mouse_down),
              ui::Button { pos: [0., -0.6], label: String::from("Quit"), click: || { *control_flow = winit::event_loop::ControlFlow::Exit; } }.gen_vertices(&mouse_pos, mouse_down)
            ]
          }.gen_vertices()
          
        },
        PauseType::Inventory => {
          ui::Group {
            children: vec![
              ui::Label { pos: [0., 0.75], text: String::from("Inventory"), size_x: tile_width, size_y: tile_height }.gen_vertices(),
              ui::Inventory { pos: [0., 0.25], size_x: tile_width, size_y: tile_height, inventory: &self.inventory }.gen_vertices(&mouse_pos)
            ]
          }.gen_vertices()
        },
        _ => {
          ui::Label { pos: [0., 0.], text: String::from("Error"), size_x: tile_width, size_y: tile_height }.gen_vertices()
        }
      };

      let index_start: u16 = vertices.len().try_into().unwrap();
      let pause_label_length: u16 = pause_ui_vertices.len().try_into().unwrap();
      let index_end: u16 = index_start + pause_label_length;
      vertices.append(&mut pause_ui_vertices);

      for n in index_start..index_end {
        indices.push(n);
      }
    }

    ( vertices.iter().cloned().collect(), indices.iter().cloned().collect() )
  }

  pub fn write_out_gamedata (&self) {
    let gamedata = GameData { health: self.health, max_health: self.max_health, seed: self.seed, x: self.x, y: self.y, time: self.uniforms.time };
    let file_string = serde_yaml::to_string(&gamedata).unwrap();
    fs::create_dir("worlds").unwrap_or(());
    fs::write(format!("worlds/{}.yaml", self.world_name), &file_string).unwrap();
  }
  pub fn read_gamedata (&self) -> GameData {
    let file_string = fs::read_to_string(format!("worlds/{}.yaml", self.world_name)).unwrap();
    fs::create_dir("worlds").unwrap_or(());
    let result: GameData = serde_yaml::from_str(&file_string).unwrap();
    println!("{:?}", result);
    result
  }
  pub fn load_gamedata (&mut self, gamedata: GameData) -> Vec<Vec<tiles::TileProperties>> {
    self.x = gamedata.x;
    self.y = gamedata.y;
    self.max_health = gamedata.max_health;
    self.health = gamedata.health;
    self.uniforms.time = gamedata.time;
    worldgen::elevation_to_tiles(worldgen::generate_perlin(1000, 1000, gamedata.seed))
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
      render::Vertex { pos: [ -tile_width, tile_height * 3. ], tex_coords: [ 0., texture_height * 4. ], animation_frames: 1. }, // top left
      render::Vertex { pos: [ -tile_width, tile_height * -3. ], tex_coords: [ 0., texture_height * 7. ], animation_frames: 1. }, // bottom left
      render::Vertex { pos: [ tile_width, tile_height * -3. ], tex_coords: [ texture_width, texture_height * 7. ], animation_frames: 1. }, // bottom right
      render::Vertex { pos: [ tile_width, tile_height * 3. ], tex_coords: [ texture_width, texture_height * 4. ], animation_frames: 1. } // top right
    ],
    vec![ 0, 1, 2, 0, 2, 3 ] // player indices
  )
}