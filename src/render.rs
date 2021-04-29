use std::{borrow::Cow, convert::TryInto, mem};

use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

const TILESET_WIDTH: i32 = 128;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Vertex {
  pub pos: [f32; 2],
  pub tex_coords: [f32; 2]
}

pub struct Render {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
}

impl Render {

  /** 
  * Create an instance of renderer
  */
  pub async fn new (window: &winit::window::Window) -> Self {

    let size = window.inner_size();
    // wgpu stuff
    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let surface = unsafe { instance.create_surface(window) };

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      // Request an adapter which can render to our surface
      compatible_surface: Some(&surface)
    }).await.expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      label: None,
      features: wgpu::Features::empty(),
      limits: wgpu::Limits::default()
    }, None).await.expect("Failed to create device");

    Render {
      surface, device, queue
    }

  }

}

pub fn gen_vertices (world: Vec<Vec<usize>>, width: i32, height: i32) -> (Vec<Vertex>, Vec<u16>) {
  // create a vector to write to
  let mut vertices: Vec<Vertex> = Vec::new();
  let mut indices: Vec<u16> = Vec::new();
  // create some math for rendering tiles
  let tile_width: f32 = 1. / width as f32;
  let tile_height: f32 = 1. / height as f32;
  let texture_size: f32 = 8. / TILESET_WIDTH as f32;
  // iterate through the tiles and generate vertices
  for (x, row) in world.iter().enumerate() {
    for (y, tiletype) in row.iter().enumerate() {
      // top left bottom left top right triangle
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top left
        pos: [ x as f32 * tile_width, 1. - (y as f32 * tile_height) ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom left
        pos: [ x as f32 * tile_width, 1. - ((y as f32 + 1.) * tile_height) ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top right
        pos: [ (x as f32 + 1.) * tile_width, 1. - (y as f32 * tile_height) ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
    }
  }
  (vertices.iter().cloned().collect(), indices.iter().cloned().collect())
}