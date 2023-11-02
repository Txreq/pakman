use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

mod collider;
mod map;
mod transform;

pub use collider::Collider;
pub use map::*;
pub use transform::*;

pub trait GameObject<'a> {
  fn get_transform(&'a self) -> &'a Transform;
  fn render(&mut self, gl: &mut GlGraphics, render_args: &RenderArgs);
  fn update(&mut self, ctx: &GameContext) {
    ()
  }
}

#[derive(Debug)]
pub struct GameContext {
  pub render_args: Option<RenderArgs>,
  pub update_args: Option<UpdateArgs>,
  pub keys_args: Option<ButtonArgs>,
}

impl GameContext {
  pub fn new() -> Self {
    Self {
      render_args: None,
      update_args: None,
      keys_args: None,
    }
  }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}
