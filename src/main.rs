extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use cgmath::num_traits::float::FloatCore;
use cgmath::num_traits::real::Real;
use cgmath::num_traits::Float;
use cgmath::Vector2;
use core::borrow;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonEvent, ButtonState, EventLoop, Key};
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

mod components;
mod utils;

use components::*;

// game
struct Game {
  gl: GlGraphics,
  player: Player,
  map: Rc<RefCell<Map>>,
  context: GameContext,
}

impl Game {
  fn start(graphics_api: GlGraphics, map: Rc<RefCell<Map>>) -> Self {
    Self {
      gl: graphics_api,
      player: Player::new(Rc::clone(&map)),
      map: Rc::clone(&map),
      context: GameContext::new(),
    }
  }
  fn render(&mut self) {
    use graphics::*;

    let render_args = self.context.render_args.unwrap();
    self.gl.draw(render_args.viewport(), |c, gl| {
      clear(graphics::color::WHITE, gl);

      self.map.borrow_mut().render(gl, &render_args);
      self.player.render(gl, &render_args);
    })
  }

  fn update(&mut self) {
    self.player.update(&self.context)
  }

  fn event_loop(&mut self) {
    if self.context.render_args.is_none()
      || self.context.update_args.is_none()
      || self.context.keys_args.is_none()
    {
      return;
    }

    self.render();
    self.update();
  }
}

// player entity
struct Player {
  transform: Transform,
  curr_map: Rc<RefCell<Map>>,
}

impl Player {
  pub fn new(curr_map: Rc<RefCell<Map>>) -> Self {
    Self {
      transform: Transform {
        x: 10.0 * TILE_SIZE,
        y: 13.0 * TILE_SIZE,
        direction: Direction::Right,
        width: 20.0,
        height: 20.0,
        velocity: Vector2::from([TILE_SIZE, TILE_SIZE]),
      },
      curr_map,
    }
  }
}

impl Collider<'_> for Player {
  fn get_transform(&self) -> &Transform {
    &self.transform
  }
}

impl GameObject<'_> for Player {
  fn get_transform(&self) -> &Transform {
    &self.transform
  }

  fn render(&mut self, gl: &mut GlGraphics, render_args: &RenderArgs) {
    use graphics::*;

    gl.draw(render_args.viewport(), |c, gl| {
      let shape = rectangle::square(self.transform.x, self.transform.y, self.transform.width);

      rectangle(color::RED, shape, c.transform, gl);
    });
  }

  fn update(&mut self, ctx: &GameContext) {
    let keys_args = ctx.keys_args.unwrap();
    let updated_args = ctx.update_args.unwrap();

    let mut new_x = self.transform.x;
    let mut new_y = self.transform.y;
    let mut new_dir = &self.transform.direction;

    if keys_args.state == ButtonState::Press {
      match keys_args.button {
        Button::Keyboard(Key::Up) => new_dir = &Direction::Up,
        Button::Keyboard(Key::Down) => new_dir = &Direction::Down,
        Button::Keyboard(Key::Right) => new_dir = &Direction::Right,
        Button::Keyboard(Key::Left) => new_dir = &Direction::Left,
        _ => {}
      }
    }

    match new_dir {
      Direction::Up => new_y -= self.transform.velocity.y * updated_args.dt,
      Direction::Down => new_y += self.transform.velocity.y * updated_args.dt,
      Direction::Right => new_x += self.transform.velocity.x * updated_args.dt,
      Direction::Left => new_x -= self.transform.velocity.x * updated_args.dt,
    }

    if self.can_move(new_x, new_y, &new_dir) {
      self.transform.x = new_x;
      self.transform.y = new_y;
      self.transform.direction = new_dir.clone();
    }
  }
}

impl Player {
  fn can_move(&self, dest_x: f64, dest_y: f64, new_direction: &Direction) -> bool {
    let map = self.curr_map.borrow();
    let mut grid_x = dest_x / TILE_SIZE;
    let mut grid_y = dest_y / TILE_SIZE;

    match new_direction {
      Direction::Up => {
        map.tiles[grid_y.floor() as usize][grid_x.floor() as usize].is_empty
          && map.tiles[grid_y.floor() as usize][grid_x.ceil() as usize].is_empty
      }
      Direction::Down => {
        map.tiles[grid_y.ceil() as usize][grid_x.floor() as usize].is_empty
          && map.tiles[grid_y.ceil() as usize][grid_x.ceil() as usize].is_empty
      }
      Direction::Right => {
        map.tiles[grid_y.floor() as usize][grid_x.ceil() as usize].is_empty
          && map.tiles[grid_y.floor() as usize][grid_x.ceil() as usize].is_empty
      }
      Direction::Left => {
        map.tiles[grid_y.floor() as usize][grid_x.floor() as usize].is_empty
          && map.tiles[grid_y.floor() as usize][grid_x.ceil() as usize].is_empty
      }
    }
  }
}

// main entry function
fn main() {
  let opengl = OpenGL::V3_2;

  let path = env::current_dir().unwrap().join("src/assets/maze.map");
  let map = Rc::new(RefCell::new(Map::new(path).expect("Failed to load maze")));

  let mut window: Window = WindowSettings::new(
    "pacman",
    [
      map.borrow().size.0 as f64 * TILE_SIZE,
      map.borrow().size.1 as f64 * TILE_SIZE,
    ],
  )
  .graphics_api(opengl)
  .exit_on_esc(true)
  .fullscreen(false)
  .resizable(false)
  .build()
  .unwrap();

  let mut game = Game::start(GlGraphics::new(opengl), map);
  let mut events = Events::new(EventSettings::new()).ups(60);

  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      game.context.render_args = Some(args.clone());
    }

    if let Some(args) = e.update_args() {
      game.context.update_args = Some(args.clone())
    }

    if let Some(args) = e.button_args() {
      game.context.keys_args = Some(args.clone())
    }

    game.event_loop();
  }
}
