use noise::{NoiseFn, Perlin, Seedable};
use crate::tiles;

/**
* Create a map of specified size with perlin noise points
*/
pub fn generate_perlin (width: i32, height: i32, seed: u32) -> Vec<Vec<(f64, f64)>> {
  // create a vector to store world data in
  let mut world: Vec<Vec<(f64, f64)>> = Vec::new();
  // make a perlin noise function to read from
  let p_noise = Perlin::new().set_seed(seed);
  // noise for vegetation
  let veg_noise = Perlin::new().set_seed(p_noise.seed() + 1);
  println!("Seed: {}", p_noise.seed());
  // loop [x][y] the size specified
  for x in 0..width {
    let mut row: Vec<(f64, f64)> = Vec::new();
    for y in 0..height {
      row.push((p_noise.get([x as f64 / 16., y as f64 / 16., 0.]), veg_noise.get([x as f64 / 1.6, y as f64 / 1.6, 0.])));
    }
    world.push(row.iter().cloned().collect());
  }

  world.iter().cloned().collect()
}

/**
* Convert perlin noise map to game tiles
* 0: Deep Ocean
* 1: Ocean
* 2: Beach
* 3: Grass
* 4: Trees
*/
pub fn elevation_to_tiles (p_map: Vec<Vec<(f64, f64)>>) -> Vec<Vec<tiles::TileProperties>> {
  // map over the map
  p_map.iter().map(|p_row| p_row.iter().map(|(tile, veg)| {
    if tile < &-0.3 { 
      if veg > &0.5 { return tiles::KELP }
      else { return tiles::DEEP_OCEAN }
    }
    else if tile < &0. { 
      if veg > &0.7 { return tiles::LILYPAD }
      else { return tiles::OCEAN }
    }
    else if tile < &0.2 { 
      if veg > &0.7 { return tiles::CACTUS }
      else { return tiles::SAND }
    }
    else { 
      if veg > &0.7 { return tiles::STUMP }
      else { return tiles::GRASS }
    }
  }).collect()).collect()
}