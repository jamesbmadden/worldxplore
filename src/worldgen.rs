use noise::{NoiseFn, Perlin, Seedable};
use rand::prelude::*;

/**
* Create a map of specified size with perlin noise points
*/
pub fn generate_perlin (width: i32, height: i32) -> Vec<Vec<f64>> {
  // create a vector to store world data in
  let mut world: Vec<Vec<f64>> = Vec::new();
  // make a perlin noise function to read from
  let p_noise = Perlin::new().set_seed(rand::random::<u32>());
  println!("Seed: {}", p_noise.seed());
  // loop [x][y] the size specified
  for x in 0..width {
    let mut row: Vec<f64> = Vec::new();
    for y in 0..height {
      row.push(p_noise.get([x as f64 / 16., y as f64 / 16., 0.]));
    }
    world.push(row.iter().cloned().collect());
  }

  world.iter().cloned().collect()
}

/**
* Convert perlin noise map to game tiles
*/
pub fn elevation_to_tiles (p_map: Vec<Vec<f64>>) -> Vec<Vec<usize>> {
  // map over the map
  p_map.iter().map(|p_row| p_row.iter().map(|tile| {
    if tile < &0. { return 0 }
    else if tile < &0.2 { return 1 }
    else { return 2 }
  }).collect()).collect()
}