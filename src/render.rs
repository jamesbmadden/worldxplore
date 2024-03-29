use std::{borrow::Cow, convert::TryInto, mem};

use crate::player;
use crate::tiles;
use crate::tiles::TileInstance;

use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

pub const TILESET_WIDTH: i32 = 128;
pub const TILESET_HEIGHT: i32 = 80;

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
  pub config: wgpu::SurfaceConfiguration,
  pub vertex_buf: wgpu::Buffer,
  pub index_buf: wgpu::Buffer,
  pub player_vertex_buf: wgpu::Buffer,
  pub player_index_buf: wgpu::Buffer,
  pub ui_vertex_buf: wgpu::Buffer,
  pub ui_index_buf: wgpu::Buffer,
  pub uniform_buf: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
  pub uniform_bind_group: wgpu::BindGroup,

  pub instances: Vec<tiles::TileInstance>,
  pub instance_buf: wgpu::Buffer,

  pub player_vertices: Vec<Vertex>,
  pub player_index_count: usize,
  pub ui_vertices: Vec<Vertex>,
  pub ui_index_count: usize,

  pub cam_width: i32,
  pub cam_height: i32,

  pub prev_x: i32,
  pub prev_y: i32,
  pub force_update: bool
}

// the vertices and indices for a single tile
const TILE_VERTICES: [Vertex; 4] = [
  Vertex { pos: [ 0., 0.], tex_coords: [ 0., 0. ], animation_frames: 0. }, // bottom left
  Vertex { pos: [ 0., 1.], tex_coords: [ 0., 8. / TILESET_HEIGHT as f32 ], animation_frames: 0. }, // top left
  Vertex { pos: [ 1., 0.], tex_coords: [ 8. / TILESET_WIDTH as f32, 0. ], animation_frames: 0. }, // bottom right
  Vertex { pos: [ 1., 1.], tex_coords: [ 8. / TILESET_WIDTH as f32, 8. / TILESET_HEIGHT as f32 ], animation_frames: 0. }, // top right
];
const TILE_INDICES: [u16; 6] = [
  2, 1, 0,
  3, 1, 2
];

impl Render {

  /** 
  * Create an instance of renderer
  */
  pub async fn new (window: &winit::window::Window, world: &mut Vec<Vec<tiles::TileProperties>>, play: &mut player::Player<'_>,  cam_width: i32, cam_height: i32) -> Self {

    let size = window.inner_size();
    // wgpu stuff
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default()
  });
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      // Request an adapter which can render to our surface
      compatible_surface: Some(&surface),
      force_fallback_adapter: false
    }).await.expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      label: None,
      features: wgpu::Features::empty(),
      limits: wgpu::Limits::default()
    }, None).await.expect("Failed to create device");

    // configure the surface
    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    // make vertex data
    let (player_vertices, player_indices) = player::player_vertices(cam_width, cam_height);
    // pass a control flow to be edited
    let mut control_flow = winit::event_loop::ControlFlow::Wait;
    let (ui_vertices, ui_indices) = play.gen_ui_vertices([0., 0.], false, &mut control_flow, world);
    let player_index_count = player_indices.len();
    let ui_index_count = ui_indices.len();

    // make instance data
    let instances = gen_tile_instances(world, 0, 0, cam_width, cam_height);

    // buffers
    let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::cast_slice(&TILE_VERTICES),
      usage: wgpu::BufferUsages::VERTEX
    });
    let player_vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Player Vertex Buffer"),
      contents: bytemuck::cast_slice(&player_vertices),
      usage: wgpu::BufferUsages::VERTEX
    });
    let ui_vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("UI Vertex Buffer"),
      contents: bytemuck::cast_slice(&ui_vertices),
      usage: wgpu::BufferUsages::VERTEX
    });
    let vertex_buffers = [wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as u64,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32
      ]
    }];

    let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(&TILE_INDICES),
      usage: wgpu::BufferUsages::INDEX
    });
    let player_index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Player Index Buffer"),
      contents: bytemuck::cast_slice(&player_indices),
      usage: wgpu::BufferUsages::INDEX
    });
    let ui_index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("UI Index Buffer"),
      contents: bytemuck::cast_slice(&ui_indices),
      usage: wgpu::BufferUsages::INDEX
    });

    // create the instance buffer
    let instance_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Instance Buffer"),
      contents: bytemuck::cast_slice(&instances),
      usage: wgpu::BufferUsages::VERTEX
    });

    let instanced_buffers = [
      // vertex buffer
      wgpu::VertexBufferLayout {
        array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
          0 => Float32x2,
          1 => Float32x2,
          2 => Float32
        ]
      },
      // instance buffer
      wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<TileInstance>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        // a tile instance has a lot of attributes, but mostly float32s
        attributes: &wgpu::vertex_attr_array![
          3 => Float32,
          4 => Float32,
          5 => Sint32,
          6 => Sint32,
          7 => Float32,
          8 => Float32,

          9 => Uint32,
          10 => Uint32,
          11 => Uint32,
          12 => Uint32,
          13 => Uint32,
          14 => Float32,
          15 => Float32
        ],
      }
    ];

    // uniform only (as of now at least) contains the offset for movement.
    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Uniform Buffer"),
      contents: bytemuck::cast_slice(&[player::Uniforms::default()]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });
    let uniform_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX,
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
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
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
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      view_formats: &[],
      label: Some("Tileset")
    });

    queue.write_texture(
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All
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
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true }
          },
          count: None
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &instanced_buffers
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL
        })]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None
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
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL
        })]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None
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
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL
        })]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None
    });

    Render {
      surface, device, queue, render_pipeline, player_render_pipeline, ui_render_pipeline, bind_group, uniform_bind_group,
      vertex_buf, index_buf, player_vertex_buf, player_index_buf, ui_vertex_buf, ui_index_buf, uniform_buf,
      instances, instance_buf, player_index_count, player_vertices, ui_index_count, ui_vertices,
      cam_width, cam_height, config,
      prev_x: 0, prev_y: 0, force_update: false
    }

  }

  /**
  * Update vertices based on current camera position
  */
  pub fn update (&mut self, world: &mut Vec<Vec<tiles::TileProperties>>, player: &mut player::Player, mouse_pos: [f32; 2], mouse_down: bool, control_flow: &mut winit::event_loop::ControlFlow) {
    // update the camera
    player.update(world, self.cam_width, self.cam_height);

    // only update the tiles if the game is paused
    if !player.paused {
      // round cam position to nearest tile
      let rounded_x = player.x.floor() as i32;
      let rounded_y = player.y.floor() as i32;

      // check if values need update
      if rounded_x != self.prev_x || rounded_y != self.prev_y || self.force_update {
        // if so, update local values
        let instances = gen_tile_instances(&world, rounded_x, rounded_y, self.cam_width, self.cam_height);
        self.instances = instances;

        self.instance_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&self.instances),
          usage: wgpu::BufferUsages::VERTEX
        });
      }
      // update the uniforms buffer with new data
      self.queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(&[player.uniforms]));
      // update previous position
      self.prev_x = rounded_x;
      self.prev_y = rounded_y;
      self.force_update = false;
    }


    // update UI vertices
    let (ui_vertices, ui_indices) = player.gen_ui_vertices(mouse_pos, mouse_down, control_flow, world);
    self.ui_vertices = ui_vertices;
    self.ui_index_count = ui_indices.len();
    self.ui_vertex_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("UI Vertex Buffer"),
      contents: bytemuck::cast_slice(&self.ui_vertices),
      usage: wgpu::BufferUsages::VERTEX
    });
    self.ui_index_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("UI Index Buffer"),
      contents: bytemuck::cast_slice(&ui_indices),
      usage: wgpu::BufferUsages::INDEX
    });

    
  }

  /**
  * Render vertices to screen
  */
  pub fn render (&self)-> Result<(), wgpu::SurfaceError> {
    
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    {

      let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.,
              g: 0.,
              b: 0.,
              a: 1.0,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });

      // render world
      rpass.set_pipeline(&self.render_pipeline);
      rpass.set_bind_group(0, &self.bind_group, &[]);
      rpass.set_bind_group(1, &self.uniform_bind_group, &[]);
      rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
      rpass.set_vertex_buffer(1, self.instance_buf.slice(..));
      rpass.draw_indexed(0..TILE_INDICES.len() as _, 0, 0..self.instances.len() as _);
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

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }

  pub fn resize (&mut self, size: winit::dpi::PhysicalSize<u32>) {
    // update the camera width
    let new_cam_height: i32 = 24;
    // multiplication must be done as floats then converted back to an integer
    let new_cam_width: i32 = ((size.width as f32 / size.height as f32) * new_cam_height as f32) as i32;
    // one should be added to include offscreen tiles
    self.cam_height = new_cam_height + 1;
    self.cam_width = new_cam_width + 1;
    // create new swap chain
    if size.width > 0 && size.height > 0 {
      self.config.width = size.width;
      self.config.height = size.height;
      self.surface.configure(&self.device, &self.config);
  }

    self.force_update = true;
  }

}

/**
 * Generate instances of tiles
 */
pub fn gen_tile_instances (world: &Vec<Vec<tiles::TileProperties>>, start_x: i32, start_y: i32, width: i32, height: i32) -> Vec<tiles::TileInstance> {

  // create a vector to write to
  let mut instances: Vec<tiles::TileInstance> = Vec::new();
  // create some math for rendering tiles
  let tile_width: f32 = 1. / width as f32;
  let tile_height: f32 = 1. / height as f32;
  let texture_width: f32 = 8. / TILESET_WIDTH as f32;
  let texture_height: f32 = 8. / TILESET_HEIGHT as f32;

  for x in start_x..(start_x + width + 1) {
    for y in start_y..(start_y + height + 1) {

      // relative position on the screen
      let relative_x = (x - start_x) as f32;
      let relative_y = (y - start_y) as f32;

      // find the type of tile for this instance
      let tile_type = world[x as usize][y as usize];

      instances.push(TileInstance {
        x: relative_x, y: relative_y, // the position on the screen it fills
        offset_x: tile_type.offset_x, offset_y: tile_type.offset_y,
        height: tile_type.height, width: tile_type.width,
        ts_coord_x: tile_type.ts_coord_x, ts_coord_y: tile_type.ts_coord_y,
        animation_frames: tile_type.animation_frames,

        // general information about the tiles for rendering
        tile_width, tile_height,
        tx_width: texture_width, tx_height: texture_height
      });

    }
  }

  return instances;

}