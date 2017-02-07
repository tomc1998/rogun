use entity::{EntityID, Component};
use glium::glutin::VirtualKeyCode;

/// Controller to control left / right movement control
/// Dependencies:
/// CompBody
pub struct CompLRController {
  pub entity_id: EntityID,
  pub key_l: VirtualKeyCode,
  pub key_r: VirtualKeyCode,

  pub force: f32,
}
impl Component for CompLRController {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

/// Component to control jumping
/// Dependencies:
/// CompBody
/// CompCollAABB (For resetting jump counter)
pub struct CompJumpController {
  pub entity_id: EntityID,
  pub key_jump: VirtualKeyCode,

  /// Speed to set entity to on jump
  pub jump_speed: VirtualKeyCode,

  /// Number of jumps left - resets on touching the ground.
  pub num_jumps: u8,

  /// Maximum jumps left. Set to 0 for infinite jumps.
  pub max_jumps: u8,
}
impl Component for CompJumpController {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}
