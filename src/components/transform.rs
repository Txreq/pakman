use super::Direction;
use cgmath::Vector2;

#[derive(Debug, Clone)]
pub struct Transform {
  pub x: f64,
  pub y: f64,
  pub direction: Direction,
  pub width: f64,
  pub height: f64,
  pub velocity: Vector2<f64>,
}

pub trait Rect {
  fn x(&self) -> f64;
  fn y(&self) -> f64;
  fn w(&self) -> f64;
  fn h(&self) -> f64;
  fn xw(&self) -> f64;
  fn yh(&self) -> f64;
}

impl Rect for Transform {
  fn x(&self) -> f64 {
    self.x
  }

  fn y(&self) -> f64 {
    self.y
  }

  fn w(&self) -> f64 {
    self.width
  }

  fn h(&self) -> f64 {
    self.height
  }

  fn xw(&self) -> f64 {
    self.x + self.width
  }

  fn yh(&self) -> f64 {
    self.y + self.height
  }
}
