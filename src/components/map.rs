extern crate graphics;

use cgmath::Vector2;

use super::*;
use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

pub const TILE_SIZE: f64 = 30.0;

#[derive(Debug, Clone)]
pub struct Tile {
  pub is_empty: bool,
  pub transform: Transform,
}

#[derive(Debug, Clone)]
pub struct Map {
  pub tiles: Vec<Vec<Tile>>,
  pub size: (usize, usize),
}

impl Map {
  pub fn render(&mut self, gl: &mut GlGraphics, render_args: &RenderArgs) {
    use graphics::*;
    gl.draw(render_args.viewport(), |c, gl| {
      self.tiles.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, tile)| {
          rectangle(
            match tile.is_empty {
              true => color::WHITE,
              false => color::BLACK,
            },
            rectangle::rectangle_by_corners(
              tile.transform.x,
              tile.transform.y,
              tile.transform.xw(),
              tile.transform.yh(),
            ),
            c.transform,
            gl,
          )
        })
      })
    })
  }
}

impl Map {
  pub fn new(map_ascii_map: PathBuf) -> Result<Self> {
    let mut content = String::new();

    let mut map_file = File::open(Path::new(&map_ascii_map)).expect("Failed to open maze map");

    match map_file.read_to_string(&mut content) {
      Ok(_) => {
        let tiles: Vec<Vec<Tile>> = content
          .split("\n")
          .into_iter()
          .enumerate()
          .map(|(y, line)| {
            line
              .chars()
              .into_iter()
              .enumerate()
              .map(|(x, num_char)| {
                let num: u8 = String::from(num_char)
                  .parse()
                  .expect("Failed to parse bin tile");

                Tile {
                  is_empty: num == 0,
                  transform: Transform {
                    x: x as f64 * TILE_SIZE as f64,
                    y: y as f64 * TILE_SIZE as f64,
                    direction: Direction::Up,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                    velocity: Vector2::from([0.0, 0.0]),
                  },
                }
              })
              .collect()
          })
          .collect();

        Ok(Self {
          size: (tiles[0].len(), tiles.len()),
          tiles,
        })
      }
      Err(_) => panic!("Failed to read to string"),
    }
  }
}
