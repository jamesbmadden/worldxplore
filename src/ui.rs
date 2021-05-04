use crate::render;

use std::convert::TryInto;

pub trait GenVertices {
  fn gen_vertices (&self) -> Vec<render::Vertex>;
}

pub const TILE_WIDTH: f32 = 8. / render::TILESET_WIDTH as f32;
pub const TILE_HEIGHT: f32 = 8. / render::TILESET_HEIGHT as f32;

pub struct Group {
  pub children: Vec<Vec<render::Vertex>> // will later be changed to actually include any object
}

impl Group {

  pub fn gen_vertices (&self) -> Vec<render::Vertex> {
    self.children.iter().flat_map(|child| -> Vec<render::Vertex> {child.iter().cloned().collect()}).collect()
  }
  
}

pub struct Button<F> where F: FnMut() {
  pub label: String,
  pub pos: [f32; 2],
  pub click: F
}

impl<F> Button<F> where F: FnMut() {

  pub fn gen_vertices (&mut self, mouse_pos: &[f32; 2], mouse_down: bool) -> Vec<render::Vertex> {

    let mut vertices: Vec<render::Vertex> = Vec::new();
    // add button texture at specified position
    let start_x = self.pos[0] - TILE_WIDTH * 2.;
    let start_y = self.pos[1] + (TILE_HEIGHT / 2.);
    let end_x = start_x + TILE_WIDTH * 4.;
    let end_y = start_y - TILE_HEIGHT;
    let mut tex_coords: [[f32; 2]; 4] = [
      [TILE_WIDTH * 6., TILE_HEIGHT * 4.],
      [TILE_WIDTH * 6., TILE_HEIGHT * 5.],
      [TILE_WIDTH * 10., TILE_HEIGHT * 5.],
      [TILE_WIDTH * 10., TILE_HEIGHT * 4.]
    ];


    // check if mouse is over button
    if start_x <= mouse_pos[0] && mouse_pos[0] <= end_x && start_y >= mouse_pos[1] && mouse_pos[1] >= end_y {
      // move the tex coords over to hover style
      tex_coords[0][0] += TILE_WIDTH * 4.;
      tex_coords[1][0] += TILE_WIDTH * 4.;
      tex_coords[2][0] += TILE_WIDTH * 4.;
      tex_coords[3][0] += TILE_WIDTH * 4.;
      // check if the button is being clicked
      if mouse_down {
        // run the closure
        (self.click)();
      }
    }

    vertices.push(render::Vertex { pos: [ start_x, start_y ], tex_coords: tex_coords[0], animation_frames: 1. }); // top left
    vertices.push(render::Vertex { pos: [ start_x, end_y ], tex_coords: tex_coords[1], animation_frames: 1. }); // bottom left
    vertices.push(render::Vertex { pos: [ end_x, end_y ], tex_coords: tex_coords[2], animation_frames: 1. }); // bottom right
    vertices.push(render::Vertex { pos: [ start_x, start_y ], tex_coords: tex_coords[0], animation_frames: 1. }); // top left
    vertices.push(render::Vertex { pos: [ end_x, end_y ], tex_coords: tex_coords[2], animation_frames: 1. }); // bottom right
    vertices.push(render::Vertex { pos: [ end_x, start_y ], tex_coords: tex_coords[3], animation_frames: 1. }); // top right
    
    // add label at same position
    vertices.append(&mut Label { pos: self.pos, text: self.label.clone(), size_x: TILE_WIDTH / 2., size_y: TILE_HEIGHT / 2. }.gen_vertices());
    // return vertices cloned
    vertices.iter().cloned().collect()
  }

}

pub struct Label {
  pub text: String,
  pub pos: [f32; 2],
  pub size_x: f32,
  pub size_y: f32
}

impl Label {

  pub fn write (text: String, pos: [f32; 2], size_x: f32, size_y: f32) -> (Vec<render::Vertex>, Vec<u16>) {
    let vertices = Label { text, pos, size_x, size_y }.gen_vertices();
    let indices: Vec<u16> = (0_u16..(vertices.len() - 1) as u16).collect();

    (vertices.iter().cloned().collect(), indices.iter().cloned().collect())
  }

  pub fn gen_vertices (&self) -> Vec<render::Vertex> {
    
    let mut vertices: Vec<render::Vertex> = Vec::new();
    let string_length = self.text.len();
    let start_x: f32 = self.pos[0] - (string_length as f32 * self.size_x / 2.);
    let start_y: f32 = self.pos[1] + self.size_y / 2.;
    // iterate through the characters in our string and return text as vertices
    for (i, character) in self.text.chars().enumerate() {
      // get the character position on screen
      let char_x = start_x + self.size_x * i as f32;
      let char_y = start_y;
      // get the texture position in the tileset
      let tex_position = match character {
        'A' => [TILE_WIDTH * 2., TILE_HEIGHT * 5.],
        'B' => [TILE_WIDTH * 3., TILE_HEIGHT * 5.],
        'C' => [TILE_WIDTH * 4., TILE_HEIGHT * 5.],
        'D' => [TILE_WIDTH * 5., TILE_HEIGHT * 5.],
        'E' => [TILE_WIDTH * 6., TILE_HEIGHT * 5.],
        'F' => [TILE_WIDTH * 7., TILE_HEIGHT * 5.],
        'G' => [TILE_WIDTH * 8., TILE_HEIGHT * 5.],
        'H' => [TILE_WIDTH * 9., TILE_HEIGHT * 5.],
        'I' => [TILE_WIDTH * 10., TILE_HEIGHT * 5.],
        'J' => [TILE_WIDTH * 11., TILE_HEIGHT * 5.],
        'K' => [TILE_WIDTH * 12., TILE_HEIGHT * 5.],
        'L' => [TILE_WIDTH * 13., TILE_HEIGHT * 5.],
        'M' => [TILE_WIDTH * 14., TILE_HEIGHT * 5.],
        'N' => [TILE_WIDTH * 15., TILE_HEIGHT * 5.],
        'O' => [TILE_WIDTH * 2., TILE_HEIGHT * 6.],
        'P' => [TILE_WIDTH * 3., TILE_HEIGHT * 6.],
        'Q' => [TILE_WIDTH * 4., TILE_HEIGHT * 6.],
        'R' => [TILE_WIDTH * 5., TILE_HEIGHT * 6.],
        'S' => [TILE_WIDTH * 6., TILE_HEIGHT * 6.],
        'T' => [TILE_WIDTH * 7., TILE_HEIGHT * 6.],
        'U' => [TILE_WIDTH * 8., TILE_HEIGHT * 6.],
        'V' => [TILE_WIDTH * 9., TILE_HEIGHT * 6.],
        'W' => [TILE_WIDTH * 10., TILE_HEIGHT * 6.],
        'X' => [TILE_WIDTH * 11., TILE_HEIGHT * 6.],
        'Y' => [TILE_WIDTH * 12., TILE_HEIGHT * 6.],
        'Z' => [TILE_WIDTH * 13., TILE_HEIGHT * 6.],
        'a' => [0., TILE_HEIGHT * 7.],
        'b' => [TILE_WIDTH, TILE_HEIGHT * 7.],
        'c' => [TILE_WIDTH * 2., TILE_HEIGHT * 7.],
        'd' => [TILE_WIDTH * 3., TILE_HEIGHT * 7.],
        'e' => [TILE_WIDTH * 4., TILE_HEIGHT * 7.],
        'f' => [TILE_WIDTH * 5., TILE_HEIGHT * 7.],
        'g' => [TILE_WIDTH * 6., TILE_HEIGHT * 7.],
        'h' => [TILE_WIDTH * 7., TILE_HEIGHT * 7.],
        'i' => [TILE_WIDTH * 8., TILE_HEIGHT * 7.],
        'j' => [TILE_WIDTH * 9., TILE_HEIGHT * 7.],
        'k' => [TILE_WIDTH * 10., TILE_HEIGHT * 7.],
        'l' => [TILE_WIDTH * 11., TILE_HEIGHT * 7.],
        'm' => [TILE_WIDTH * 12., TILE_HEIGHT * 7.],
        'n' => [TILE_WIDTH * 13., TILE_HEIGHT * 7.],
        'o' => [TILE_WIDTH * 14., TILE_HEIGHT * 7.],
        'p' => [TILE_WIDTH * 15., TILE_HEIGHT * 7.],
        'q' => [0., TILE_HEIGHT * 8.],
        'r' => [TILE_WIDTH, TILE_HEIGHT * 8.],
        's' => [TILE_WIDTH * 2., TILE_HEIGHT * 8.],
        't' => [TILE_WIDTH * 3., TILE_HEIGHT * 8.],
        'u' => [TILE_WIDTH * 4., TILE_HEIGHT * 8.],
        'v' => [TILE_WIDTH * 5., TILE_HEIGHT * 8.],
        'w' => [TILE_WIDTH * 6., TILE_HEIGHT * 8.],
        'x' => [TILE_WIDTH * 7., TILE_HEIGHT * 8.],
        'y' => [TILE_WIDTH * 8., TILE_HEIGHT * 8.],
        'z' => [TILE_WIDTH * 9., TILE_HEIGHT * 8.],
        '0' => [TILE_WIDTH * 10., TILE_HEIGHT * 8.],
        '1' => [TILE_WIDTH * 11., TILE_HEIGHT * 8.],
        '2' => [TILE_WIDTH * 12., TILE_HEIGHT * 8.],
        '3' => [TILE_WIDTH * 13., TILE_HEIGHT * 8.],
        '4' => [TILE_WIDTH * 14., TILE_HEIGHT * 8.],
        '5' => [TILE_WIDTH * 15., TILE_HEIGHT * 8.],
        '6' => [0., TILE_HEIGHT * 9.],
        '7' => [TILE_WIDTH, TILE_HEIGHT * 9.],
        '8' => [TILE_WIDTH * 2., TILE_HEIGHT * 9.],
        '9' => [TILE_WIDTH * 3., TILE_HEIGHT * 9.],
        _ => [TILE_WIDTH, TILE_HEIGHT * 4.]
      };
      // now we can generate the vertices  - we don't give indices because it is up to the root UI element to just
      // make a list based on a (0..(vertices.len() - 1)).collect() so that multiple UI Elements can be easily combined
      // first tri: top left bottom left bottom right
      vertices.push(render::Vertex { pos: [char_x, char_y], tex_coords: tex_position, animation_frames: 1. }); // top left
      vertices.push(render::Vertex { pos: [char_x, char_y - self.size_y], tex_coords: [tex_position[0], tex_position[1] + TILE_HEIGHT], animation_frames: 1. }); // bottom left
      vertices.push(render::Vertex { pos: [char_x + self.size_x, char_y - self.size_y], tex_coords: [tex_position[0] + TILE_WIDTH, tex_position[1] + TILE_HEIGHT], animation_frames: 1. }); // bottom right
      // second tri: top left bottom right top right
      vertices.push(render::Vertex { pos: [char_x, char_y], tex_coords: tex_position, animation_frames: 1. }); // top left
      vertices.push(render::Vertex { pos: [char_x + self.size_x, char_y - self.size_y], tex_coords: [tex_position[0] + TILE_WIDTH, tex_position[1] + TILE_HEIGHT], animation_frames: 1. }); // bottom right
      vertices.push(render::Vertex { pos: [char_x + self.size_x, char_y], tex_coords: [tex_position[0] + TILE_WIDTH, tex_position[1]], animation_frames: 1. }); // top right

    }

    vertices.iter().cloned().collect()

  }

}