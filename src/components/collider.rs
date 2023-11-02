use super::{transform::Transform, GameObject};

pub trait Collider<'a> {
  fn intersects(&'a self, transfrom: &Transform) -> bool {
    let self_transform = self.get_transform();

    if self_transform.x < transfrom.x + transfrom.width
      && self_transform.x + self_transform.width > transfrom.x
      && self_transform.y < transfrom.y + transfrom.height
      && self_transform.y + self_transform.height > transfrom.y
    {
      return true;
    }

    false
  }

  fn collide<T>(&'a self, game_obj: &'a T) -> bool
  where
    T: GameObject<'a>,
  {
    self.intersects(game_obj.get_transform())
  }

  fn get_transform(&'a self) -> &'a Transform;
}
