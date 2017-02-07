use entity::{EntityID, Component};
use common::vec::Vec2f32;

/// Position component for entity. Defines a 2D view position and size.
#[derive(Clone)]
pub struct CompAABB {
  pub entity_id: EntityID,
  pub pos: Vec2f32,
  pub size: Vec2f32,
}
impl Component for CompAABB {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

/// Entity has a 'body' which responds to the environment
/// Dependencies:
/// CompAABB
pub struct CompBody {
  pub entity_id: EntityID,
  pub vel: Vec2f32,
  pub acc: Vec2f32,

  /// Force is like acc, but it gets cleared after every frame. Apply a force
  /// every frame in order to get a constant acceleration.
  pub force: Vec2f32,

  /// Used when applying forces.
  pub mass: f32,

  /// Maximum speed cap on body. 
  pub max_speed: f32,
}
impl Component for CompBody {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

impl CompBody {
  pub fn apply_force(&mut self, force: Vec2f32) {
    self.force.0 += self.mass * force.0;
    self.force.1 += self.mass * force.1;
  }
  pub fn clear_force(&mut self) {
    self.force.0 = 0.0;
    self.force.1 = 0.0;
  }
}

