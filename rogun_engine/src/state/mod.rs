use entity::component_core::CompAABB;
use entity::component_render::CompDebugDraw;

pub mod comp_list;

use self::comp_list::CompList;

pub struct GameState {
  pub comp_debug_draw: CompList<CompDebugDraw>,
  pub comp_aabb: CompList<CompAABB>,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      comp_debug_draw: CompList::new(),
      comp_aabb: CompList::new(),
    }
  }
}
