use std::collections::HashSet;

pub struct Camera {
  pub keys_down: HashSet<winit::event::VirtualKeyCode>,
  pub width: i32,
  pub height: i32,
  pub x: f32,
  pub y: f32
}

impl Camera {

  pub fn new (width: i32, height: i32) -> Self {
    Camera {
      keys_down: HashSet::new(),
      x: 0., y: 0.,
      width, height
    }
  }

  // if keys are pressed, update x and y values
  pub fn update (&mut self) {
    let speed = 0.5;

    if self.keys_down.contains(&winit::event::VirtualKeyCode::A) {
      self.x -= speed;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::D) {
      self.x += speed;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::S) {
      self.y += speed;
    }
    if self.keys_down.contains(&winit::event::VirtualKeyCode::W) {
      self.y -= speed;
    }

    // prevent from raising or lowering the x or y past bounds
    if self.x < 0. {
      self.x = 0.;
    }
    if self.y < 0. {
      self.y = 0.;
    }
  }

  // key pressed, add it to keys down
  pub fn key_pressed (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.insert(key);
  }
  // key released, add it to keys down
  pub fn key_released (&mut self, key: winit::event::VirtualKeyCode) {
    self.keys_down.remove(&key);
  }

}