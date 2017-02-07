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
}
impl Component for CompBody {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

