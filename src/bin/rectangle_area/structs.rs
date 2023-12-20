#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

impl Rectangle {
    pub fn create_square(size: u32) -> Self { Self { width: size, height: size } }
    pub fn calculate_area(&self) -> u32 { self.height * self.width }
    pub fn is_width_valid(&self) -> bool { self.width > 0 }
    pub fn is_height_valid(&self) -> bool { self.height > 0 }
    pub fn can_hold_rectangle(&self, rect: &Rectangle) -> bool { self.height > rect.height && self.width > rect.width }
}