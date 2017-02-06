use std::ops::*;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Vec2f32 (pub f32, pub f32);

impl Vec2f32 {
  fn new() -> Vec2f32 { Vec2f32(0.0, 0.0) }
}

impl AddAssign for Vec2f32 {
  fn add_assign(&mut self, other: Vec2f32) {
    self.0 += other.0;
    self.1 += other.1;
  }
}

impl Add for Vec2f32 {
  type Output = Vec2f32;
  fn add(self, other: Vec2f32) -> Self::Output {
    Vec2f32(self.0 + other.0, self.1 + other.1)
  }
}

impl SubAssign for Vec2f32 {
  fn sub_assign(&mut self, other: Vec2f32) {
    self.0 -= other.0;
    self.1 -= other.1;
  }
}

impl Sub for Vec2f32 {
  type Output = Vec2f32;
  fn sub(self, other: Vec2f32) -> Self::Output {
    Vec2f32(self.0 - other.0, self.1 - other.1)
  }
}
