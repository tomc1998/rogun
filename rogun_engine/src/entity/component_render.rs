use common::color::RGBf32;
use entity::{EntityID, Component};

/// Draws a rectangle at this entity's position over everything else. 50% opacity.
/// Dependencies: 
/// ComponentAABB
#[derive(Clone)]
pub struct CompDebugDraw {
  pub entity_id: EntityID,
  pub color:    RGBf32,
}
impl Component for CompDebugDraw {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

