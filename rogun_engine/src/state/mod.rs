use entity::component_core::*;
use entity::component_render::*;
use entity::component_collision::*;
use entity::component_control::*;

pub mod comp_list;

use self::comp_list::CompList;

pub struct GameState<'a> {
  pub comp_debug_draw: CompList<CompDebugDraw>,
  pub comp_aabb: CompList<CompAABB>,
  pub comp_body: CompList<CompBody>,
  pub comp_coll_aabb: CompList<CompCollAABB<'a>>,
  pub comp_lr_controller: CompList<CompLRController>,
  pub comp_jump_controller: CompList<CompJumpController>,
}

impl<'a> GameState<'a> {
  pub fn new() -> GameState<'a> {
    GameState {
      comp_debug_draw: CompList::new(),
      comp_aabb: CompList::new(),
      comp_body: CompList::new(),
      comp_coll_aabb: CompList::new(),
      comp_lr_controller: CompList::new(),
      comp_jump_controller: CompList::new(),
    }
  }
}
