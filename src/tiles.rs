#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TileProperties {
  pub ts_coord_x: u32,
  pub ts_coord_y: u32,
  pub animation_frames: u32,
  pub solid: bool,
  pub swimmable: bool,
  pub slowing: bool, // currently unused
  pub damaging: bool // currently unused
}

pub const DEEP_OCEAN: TileProperties = TileProperties {
  ts_coord_x: 0,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false
};
pub const OCEAN: TileProperties = TileProperties {
  ts_coord_x: 1,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false
};
pub const SAND: TileProperties = TileProperties {
  ts_coord_x: 2,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false
};
pub const GRASS: TileProperties = TileProperties {
  ts_coord_x: 3,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false
};
pub const TREE: TileProperties = TileProperties {
  ts_coord_x: 4,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: true,
  swimmable: false,
  slowing: false,
  damaging: false
};
pub const KELP: TileProperties = TileProperties {
  ts_coord_x: 5,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false
};
pub const LILYPAD: TileProperties = TileProperties {
  ts_coord_x: 6,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false
};
pub const CACTUS: TileProperties = TileProperties {
  ts_coord_x: 7,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: true,
  swimmable: false,
  slowing: false,
  damaging: true // not yet implemented
};