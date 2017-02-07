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
  use engine::tile::*;
  use glium::glutin::VirtualKeyCode;

  {
    // Edit tile bank, insert tile with id = 1
    let mut tile = Tile::new(1);
    tile.color = RGBf32::new(0.0, 1.0, 0.0);
    engine.register_tile(tile);
  }

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
    force: Vec2f32(0.0, 0.0),
    mass: 1.0,
    max_speed: 100.0,
  });

  g_state.comp_lr_controller.add_component(CompLRController {
    entity_id: EntityID(0),
    key_l : VirtualKeyCode::A,
    key_r : VirtualKeyCode::D,
    force: 100.0,
  });

  let mut map = TileMap16::new();
  for ii in 0..map.tiles.len() {
    if ii % 2 == 0  { continue; }
    map.tiles[ii] = 1;
  }
  g_state.tile_maps.push(map);
}

fn main() {
  let mut engine = engine::init().unwrap();
  setup_g_state(&mut engine);

  loop {
    engine.update();
  }
}
