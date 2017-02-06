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



