use entity::{EntityID, Component};
use common::vec::Vec2f32;
use std::ops::Fn;

/// First param = this entity's entity ID,
/// Second param = other entity's entity ID
type CollCallback = Fn(EntityID, EntityID) -> ();

/// Component for AABB (rectangular) collision box
/// Dependencies: 
/// CompAABB
/// CompBody (For solid entities, some collisions might need this, friction etc)
pub struct CompCollAABB<'a> {
  pub entity_id: EntityID,
  /// Offset to AABB
  pub offset: Vec2f32,
  pub size: Vec2f32,

  /// Whether or not the physics engine reacts to this collision area
  pub solid: bool,

  /// Optional callback for collision
  pub callback: Option<&'a CollCallback>,
}
impl<'a> Component for CompCollAABB<'a> {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}
