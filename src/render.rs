use std::{borrow::Cow, convert::TryInto, mem};

use crate::player;
use crate::tiles;

use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use image::GenericImageView;

pub const TILESET_WIDTH: i32 = 128;
pub const TILESET_HEIGHT: i32 = 56;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Vertex {
  pub pos: [f32; 2],
  pub tex_coords: [f32; 2],
  pub animation_frames: f32
}

pub struct Render {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub render_pipeline: wgpu::RenderPipeline,
  pub player_render_pipeline: wgpu::RenderPipeline,
  pub ui_render_pipeline: wgpu::RenderPipeline,
  pub swap_chain: wgpu::SwapChain,
  pub vertex_buf: wgpu::Buffer,
  pub index_buf: wgpu::Buffer,
  pub player_vertex_buf: wgpu::Buffer,
  pub player_index_buf: wgpu::Buffer,
  pub ui_vertex_buf: wgpu::Buffer,
  pub ui_index_buf: wgpu::Buffer,
  pub uniform_buf: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
  pub uniform_bind_group: wgpu::BindGroup,

  pub vertices: Vec<Vertex>,
  pub index_count: usize,
  pub player_vertices: Vec<Vertex>,
  pub player_index_count: usize,
  pub ui_vertices: Vec<Vertex>,
  pub ui_index_count: usize,

  pub cam_width: i32,
  pub cam_height: i32,

  pub prev_x: i32,
  pub prev_y: i32
}

impl Render {

  /** 
  * Create an instance of renderer
  */
  pub async fn new (window: &winit::window::Window, world: &Vec<Vec<tiles::TileProperties>>, play: &player::Player,  cam_width: i32, cam_height: i32) -> Self {

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
    let (vertices, indices) = gen_vertices(&world, 0, 0, cam_width, cam_height);
    let (player_vertices, player_indices) = player::player_vertices(cam_width, cam_height);
    let (ui_vertices, ui_indices) = play.gen_ui_vertices();
    let index_count = indices.len();
    let player_index_count = player_indices.len();
    let ui_index_count = ui_indices.len();

    // buffers
    let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::cast_slice(&vertices),
      usage: wgpu::BufferUsage::VERTEX
    });
    let player_vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Player Vertex Buffer"),
      contents: bytemuck::cast_slice(&player_vertices),
      usage: wgpu::BufferUsage::VERTEX
    });
    let ui_vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("UI Vertex Buffer"),
      contents: bytemuck::cast_slice(&ui_vertices),
      usage: wgpu::BufferUsage::VERTEX
    });
    let vertex_buffers = [wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as u64,
      step_mode: wgpu::InputStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute { // 
          format: wgpu::VertexFormat::Float32x2,
          offset: 0,
          shader_location: 0
        },
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float32x2,
          offset: mem::size_of::<[f32; 2]>() as u64,
          shader_location: 1
        },
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float32,
          offset: mem::size_of::<[f32; 4]>() as u64,
          shader_location: 2
        }
      ]
    }];

    let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(&indices),
      usage: wgpu::BufferUsage::INDEX
    });
    let player_index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Player Index Buffer"),
      contents: bytemuck::cast_slice(&player_indices),
      usage: wgpu::BufferUsage::INDEX
    });
    let ui_index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Player Index Buffer"),
      contents: bytemuck::cast_slice(&ui_indices),
      usage: wgpu::BufferUsage::INDEX
    });

    // uniform only (as of now at least) contains the offset for movement.
    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Uniform Buffer"),
      contents: bytemuck::cast_slice(&[player::Uniforms::default()]),
      usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
    });
    let uniform_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStage::VERTEX,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None
          },
          count: None
        }
      ],
      label: Some("Uniform Bind Group Layout")
    });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &uniform_bg_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: uniform_buf.as_entire_binding()
        }
      ],
      label: Some("Uniform Bind Group")
    });

    // load shaders
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
      flags: wgpu::ShaderFlags::all()
    });

    // create texture
    let tex_img_data = image::load_from_memory(include_bytes!("assets/tileset.png")).unwrap();
    let tex_img = tex_img_data.as_rgba8().unwrap();

    let tex_dimensions = tex_img.dimensions();

    let tex_size = wgpu::Extent3d {
      width: tex_dimensions.0,
      height: tex_dimensions.1,
      depth_or_array_layers: 1
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size: tex_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      // SAMPLED tells wgpu that we want to use this texture in shaders
      // COPY_DST means that we want to copy data to this texture
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
      label: Some("Tileset")
    });

    queue.write_texture(
      wgpu::ImageCopyTextureBase {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO
      },
      tex_img,
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some((4 * tex_dimensions.0).try_into().unwrap()),
        rows_per_image: None
      },
      tex_size
    );

    // create texture view and sampler
    let tex_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let tex_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    // create bind group
    let tex_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: false }
          },
          count: None
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::Sampler {
            comparison: false,
            filtering: true
          },
          count: None
        }
      ],
      label: Some("Texture Bind Group Layout")
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &tex_bg_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&tex_view)
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&tex_sampler)
        }
      ],
      label: Some("Texture Bind Group")
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[
        &tex_bg_layout,
        &uniform_bg_layout
      ],
      push_constant_ranges: &[]
    });

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

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
        targets: &[wgpu::ColorTargetState {
          format: swapchain_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrite::ALL
        }]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default()
    });
    let player_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_player",
        buffers: &vertex_buffers
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[wgpu::ColorTargetState {
          format: swapchain_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrite::ALL
        }]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default()
    });
    let ui_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_ui",
        buffers: &vertex_buffers
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[wgpu::ColorTargetState {
          format: swapchain_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrite::ALL
        }]
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
      surface, device, queue, render_pipeline, player_render_pipeline, ui_render_pipeline, swap_chain, bind_group, uniform_bind_group,
      vertex_buf, index_buf, player_vertex_buf, player_index_buf, ui_vertex_buf, ui_index_buf, uniform_buf,
      index_count, vertices, player_index_count, player_vertices, ui_index_count, ui_vertices,
      cam_width, cam_height,
      prev_x: 0, prev_y: 0
    }

  }

  /**
  * Update vertices based on current camera position
  */
  pub fn update (&mut self, world: &Vec<Vec<tiles::TileProperties>>, player: &mut player::Player) {
    // update the camera
    player.update(world);

    // round cam position to nearest tile
    let rounded_x = player.x.floor() as i32;
    let rounded_y = player.y.floor() as i32;

    // check if values need update
    if rounded_x != self.prev_x || rounded_y != self.prev_y {
      // if so, update local values
      // pass is swimming because player model depends on whether or not in water
      let (vertices, _) = gen_vertices(&world, rounded_x, rounded_y, self.cam_width, self.cam_height);
      self.vertices = vertices;
      self.vertex_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&self.vertices),
        usage: wgpu::BufferUsage::VERTEX
      });
    }
    // update the uniforms buffer with new data
    self.queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(&[player.uniforms]));
    // update previous position
    self.prev_x = rounded_x;
    self.prev_y = rounded_y;
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
        color_attachments: &[wgpu::RenderPassColorAttachment {
          view: &frame.view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: true
          }
        }],
        depth_stencil_attachment: None
      });

      // render world
      rpass.set_pipeline(&self.render_pipeline);
      rpass.set_bind_group(0, &self.bind_group, &[]);
      rpass.set_bind_group(1, &self.uniform_bind_group, &[]);
      rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
      rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
      // render player
      rpass.set_pipeline(&self.player_render_pipeline);
      rpass.set_index_buffer(self.player_index_buf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.set_vertex_buffer(0, self.player_vertex_buf.slice(..));
      rpass.draw_indexed(0..self.player_index_count as u32, 0, 0..1);
      // render UI
      rpass.set_pipeline(&self.ui_render_pipeline);
      rpass.set_index_buffer(self.ui_index_buf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.set_vertex_buffer(0, self.ui_vertex_buf.slice(..));
      rpass.draw_indexed(0..self.ui_index_count as u32, 0, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
  }

}

/**
* Generate vertices and indices for world and player
*/
pub fn gen_vertices (world: &Vec<Vec<tiles::TileProperties>>, start_x: i32, start_y: i32, width: i32, height: i32) -> (Vec<Vertex>, Vec<u16>) {
  // create a vector to write to
  let mut vertices: Vec<Vertex> = Vec::new();
  let mut indices: Vec<u16> = Vec::new();
  // create some math for rendering tiles
  let tile_width: f32 = 1. / width as f32;
  let tile_height: f32 = 1. / height as f32;
  let texture_width: f32 = 8. / TILESET_WIDTH as f32;
  let texture_height: f32 = 8. / TILESET_HEIGHT as f32;
  // iterate through the tiles and generate vertices
  // one more tile than always visible should be rendered to allow smooth movement
  for x in start_x..(start_x + width + 1) {
    for y in start_y..(start_y + height + 1) {
      let tiletype = world[x as usize][y as usize];

      let relative_x = (x - start_x) as f32;
      let relative_y = (y - start_y) as f32;
      // top left bottom left top right triangle
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top left
        pos: [ (relative_x * tile_width) * 2. - 1., 1. - (relative_y * tile_height) * 2. ],
        tex_coords: [ tiletype.ts_coord_x as f32 * texture_width, tiletype.ts_coord_y as f32 * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom left
        pos: [ (relative_x * tile_width) * 2. - 1., 1. - ((relative_y + 1.) * tile_height) * 2. ],
        tex_coords: [ tiletype.ts_coord_x as f32 * texture_width, (tiletype.ts_coord_y as f32 + 1.) * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top right
        pos: [ ((relative_x + 1.) * tile_width) * 2. - 1., 1. - (relative_y * tile_height) * 2. ],
        tex_coords: [ (tiletype.ts_coord_x as f32 + 1.) * texture_width,  tiletype.ts_coord_y as f32 * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
      // bottom left bottom right top right triangle
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom left
        pos: [ (relative_x * tile_width) * 2. - 1., 1. - ((relative_y + 1.) * tile_height) * 2. ],
        tex_coords: [ tiletype.ts_coord_x as f32 * texture_width, (tiletype.ts_coord_y as f32 + 1.) * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // bottom right
        pos: [ ((relative_x + 1.) * tile_width) * 2. - 1., 1. - ((relative_y + 1.) * tile_height) * 2. ],
        tex_coords: [ (tiletype.ts_coord_x as f32 + 1.) * texture_width, (tiletype.ts_coord_y as f32 + 1.) * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
      indices.push(vertices.len().try_into().unwrap());
      vertices.push(Vertex { // top right
        pos: [ ((relative_x + 1.) * tile_width) * 2. - 1., 1. - (relative_y * tile_height) * 2. ],
        tex_coords: [ (tiletype.ts_coord_x as f32 + 1.) * texture_width, tiletype.ts_coord_y as f32 * texture_height ],
        animation_frames: tiletype.animation_frames as f32
      });
    }
  }
  ( vertices.iter().cloned().collect(), indices.iter().cloned().collect() )
}