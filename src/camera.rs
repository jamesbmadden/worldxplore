use std::collections::HashSet;

pub struct Camera {
  pub keys_down: HashSet<winit::event::VirtualKeyCode>,
  pub width: i32,
  pub height: i32
}

impl Camera {

  pub fn new (width: i32, height: i32) -> Self {
    Camera {
      keys_down: HashSet::new(),
      width, height
    }
  }

  // key pressed, add it to keys down
  pub fn key_pressed (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.insert(key);
    println!("{:?}", self.keys_down);
  }
  // key released, add it to keys down
  pub fn key_released (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.remove(&key);
  }

}