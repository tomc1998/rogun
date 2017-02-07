/// Mod containing core components for entities.
/// Components will have their 'dependencies' listed in the documentation. If
/// their dependencies aren't found as components connected to the same entity,
/// then the engine will panic at runtime.
use std::ops::*;

/// Module containing components pertaining to rendering the entity on screen.
pub mod component_render;

/// Module contatining 'core' components, like AABB world position.
pub mod component_core;

/// Module contatining 'collision' components, to specifiy collision boxes for entities.
pub mod component_collision;

/// Module contatining 'control' components, to specify an entity's behaviour based on user input.
pub mod component_control;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityID (pub u16);

impl Add for EntityID {
  type Output = EntityID;
  fn add(self, other: EntityID) -> Self::Output {
    EntityID(self.0 + other.0)
  }
}


impl AddAssign for EntityID {
  fn add_assign(&mut self, other: EntityID) {
    self.0 += other.0;
  }
}

impl Sub for EntityID {
  type Output = EntityID;
  fn sub(self, other: EntityID) -> Self::Output {
    EntityID(self.0 - other.0)
  }
}


impl SubAssign for EntityID {
  fn sub_assign(&mut self, other: EntityID) {
    self.0 -= other.0;
  }
}


pub trait Component {
  fn get_entity_id(&self) -> EntityID;
}


