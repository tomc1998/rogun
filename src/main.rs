extern crate engine;

fn setup_g_state(engine: &mut engine::LibState) {
  use engine::entity::component_core::*;
  use engine::entity::component_render::*;
  use engine::entity::*;
  use engine::common::vec::*;
  use engine::common::color::*;
  let g_state = &mut engine.curr_g_state.as_mut().unwrap();

  g_state.comp_aabb.add_component(CompAABB {
    entity_id: EntityID(0),
    pos: Vec2f32(0.0, 0.0),
    size: Vec2f32(100.0, 100.0),
  });

  g_state.comp_debug_draw.add_component(CompDebugDraw {
    entity_id: EntityID(0),
    color: RGBf32::new(1.0, 0.0, 0.0),
  });
}

fn main() {
  let mut engine = engine::init().unwrap();
  setup_g_state(&mut engine);

  loop {
    engine.update();
  }
}
