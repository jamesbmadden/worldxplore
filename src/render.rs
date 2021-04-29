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
  pub render_pipeline: wgpu::RenderPipeline,
  pub swap_chain: wgpu::SwapChain,
  pub vertex_buf: wgpu::Buffer,
  pub index_buf: wgpu::Buffer,

  pub vertices: Vec<Vertex>,
  pub index_count: usize,
}

impl Render {

  /** 
  * Create an instance of renderer
  */
  pub async fn new (window: &winit::window::Window, world: Vec<Vec<usize>>) -> Self {

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

    // make vertex data
    let (vertices, indices) = gen_vertices(&world, world.len().try_into().unwrap(), world[0].len().try_into().unwrap());
    let index_count = indices.len();

    // buffers
    let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::cast_slice(&vertices),
      usage: wgpu::BufferUsage::VERTEX
    });
    let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(&indices),
      usage: wgpu::BufferUsage::INDEX
    });
    let vertex_buffers = [wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as u64,
      step_mode: wgpu::InputStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute { // 
          format: wgpu::VertexFormat::Float2,
          offset: 0,
          shader_location: 0
        },
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float2,
          offset: mem::size_of::<[f32; 2]>() as u64,
          shader_location: 1
        }
      ]
    }];
    // load shader
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
      flags: wgpu::ShaderFlags::all()
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[],
      push_constant_ranges: &[]
    });

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface);

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &vertex_buffers
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[swapchain_format.into()]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default()
    });

    let swap_chain = device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
      format: swapchain_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Mailbox
    });

    Render {
      surface, device, queue, vertex_buf, index_buf, render_pipeline, swap_chain,
      index_count, vertices
    }

  }

  /**
  * Render vertices to screen
  */
  pub fn render (&self) {
    let frame = self.swap_chain.get_current_frame().expect("Failed to acquire next swap chain texture").output;
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
      let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
          attachment: &frame.view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: true
          }
        }],
        depth_stencil_attachment: None
      });
      rpass.set_pipeline(&self.render_pipeline);
      rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
      rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
  }

}

pub fn gen_vertices (world: &Vec<Vec<usize>>, width: i32, height: i32) -> (Vec<Vertex>, Vec<u16>) {
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
        pos: [ (x as f32 * tile_width) * 2. - 1., 1. - (y as f32 * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom left
        pos: [ (x as f32 * tile_width) * 2. - 1., 1. - ((y as f32 + 1.) * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top right
        pos: [ ((x as f32 + 1.) * tile_width) * 2. - 1., 1. - (y as f32 * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      // bottom left bottom right top right triangle
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom left
        pos: [ (x as f32 * tile_width) * 2. - 1., 1. - ((y as f32 + 1.) * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom right
        pos: [ ((x as f32 + 1.) * tile_width) * 2. - 1., 1. - ((y as f32 + 1.) * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top right
        pos: [ ((x as f32 + 1.) * tile_width) * 2. - 1., 1. - (y as f32 * tile_height) * 2. ],
        tex_coords: [ *tiletype as f32 * texture_size, 1. ]
      });
    }
  }
  (vertices.iter().cloned().collect(), indices.iter().cloned().collect())
}