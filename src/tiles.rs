use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TileProperties {
  pub ts_coord_x: u32,
  pub ts_coord_y: u32,
  pub animation_frames: u32,
  pub solid: bool,
  pub swimmable: bool,
  pub slowing: bool, // currently unused
  pub damaging: bool, // currently unused

  // properties for a bigger tile (like trees):
  pub width: u32, // how many tiles the object should take up
  pub height: u32,
  pub offset_x: i32, // how much to adjust the tile from its origin for rendering
  pub offset_y: i32
}

// for a tile to be rendered, it requires the following data:
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct TileInstance {
  // position on screen it's drawn
  pub x: f32,
  pub y: f32,
  pub offset_x: i32, // how much to adjust the tile from its origin for rendering
  pub offset_y: i32,
  pub tile_width: f32,
  pub tile_height: f32,

  // texture information
  pub ts_coord_x: u32,
  pub ts_coord_y: u32,
  pub animation_frames: u32,
  pub width: u32, // how many tiles the object should take up
  pub height: u32,
  pub tx_width: f32,
  pub tx_height: f32
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum ItemTypes {
  Weapon,
  Tool,
  Resource
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ItemProperties<'a> {
  pub ts_coord_x: u32,
  pub ts_coord_y: u32,
  pub animation_frames: u32,
  pub name: &'a str,
  pub stackable: bool,
  pub item_type: ItemTypes
}

/**
 * Game tile properties
 */
pub const DEEP_OCEAN: TileProperties = TileProperties {
  ts_coord_x: 0,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const OCEAN: TileProperties = TileProperties {
  ts_coord_x: 1,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const SAND: TileProperties = TileProperties {
  ts_coord_x: 2,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const GRASS: TileProperties = TileProperties {
  ts_coord_x: 3,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const STUMP: TileProperties = TileProperties {
  ts_coord_x: 4,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: true,
  swimmable: false,
  slowing: false,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const KELP: TileProperties = TileProperties {
  ts_coord_x: 5,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: true,
  slowing: true,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const LILYPAD: TileProperties = TileProperties {
  ts_coord_x: 6,
  ts_coord_y: 0,
  animation_frames: 4,
  solid: false,
  swimmable: false,
  slowing: false,
  damaging: false,
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};
pub const CACTUS: TileProperties = TileProperties {
  ts_coord_x: 7,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: true,
  swimmable: false,
  slowing: false,
  damaging: true, // not yet implemented
  width: 1,
  height: 1,
  offset_x: 0,
  offset_y: 0
};

/**
 * Big objects
 */
pub const TREE: TileProperties = TileProperties {
  ts_coord_x: 8,
  ts_coord_y: 0,
  animation_frames: 1,
  solid: true,
  swimmable: false,
  slowing: false,
  damaging: false,
  width: 3, // because the tree takes up multiple tiles, it needs a width, height, and texture offset
  height: 4,
  offset_x: -1,
  offset_y: -3
};

/**
 * Item properties
 */

pub const STICK: ItemProperties = ItemProperties {
  ts_coord_x: 4,
  ts_coord_y: 1,
  animation_frames: 1,
  name: "Stick",
  stackable: false,
  item_type: ItemTypes::Resource
};

pub const SWORD: ItemProperties = ItemProperties {
  ts_coord_x: 4,
  ts_coord_y: 2,
  animation_frames: 1,
  name: "Sword",
  stackable: false,
  item_type: ItemTypes::Weapon
};