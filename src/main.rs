extern crate engine;
extern crate glium;

fn setup_g_state(engine: &mut engine::LibState) {
  use engine::entity::component_core::*;
  use engine::entity::component_render::*;
  use engine::entity::component_control::*;
  use engine::entity::component_collision::*;
  use engine::entity::*;
  use engine::common::vec::*;
  use engine::common::color::*;
  use glium::glutin::VirtualKeyCode;
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

  g_state.comp_body.add_component(CompBody {
    entity_id: EntityID(0),
    vel: Vec2f32(0.0, 0.0),
    acc: Vec2f32(0.0, 0.0),
  });

  g_state.comp_lr_controller.add_component(CompLRController {
    entity_id: EntityID(0),
    key_l : VirtualKeyCode::A,
    key_r : VirtualKeyCode::D,
    acceleration: 0.01,
    max_speed: 15.0,
  });
}

fn main() {
  let mut engine = engine::init().unwrap();
  setup_g_state(&mut engine);

  loop {
    engine.update();
  }
}
