#[macro_use]
extern crate glium;
extern crate time;

/// Logger module.
#[allow(dead_code)]
pub mod logger;

/// Renders the game.
pub mod renderer;

/// Commong module, contains common functions / structs like color / vec.
pub mod common;

/// Entity module. Contains component definitions for the ECS
pub mod entity;

/// State module. Holds current game state
pub mod state;

/// Input processing system 
pub mod input;

/// Physics system
pub mod physics;

/// Tile system
pub mod tile;

pub fn init<'a>() -> Option<LibState<'a>> {
  use glium::DisplayBuild;
  let display = glium::glutin::WindowBuilder::new()
    .with_gl(glium::glutin::GlRequest::Specific(
        glium::glutin::Api::OpenGl, (3, 0)))
    .build_glium().unwrap();

  let win_size;
  {
    let win_ref = display.get_window();
    if win_ref.is_none() { 
      return None;
    }
    let win_ref = win_ref.unwrap();
    let size_opt = win_ref.get_inner_size();
    if size_opt.is_none() {
      return None;
    }
    win_size = size_opt.unwrap();
  }
  let (w, h) = win_size;

  Some(LibState {
    renderer: renderer::Renderer::new(&display, w, h),
    input_system: input::InputSystem::new(),
    physics_system: physics::PhysicsSystem::new(),
    display: display,
    engine_logger: logger::Logger::new(),
    curr_g_state: Some(state::GameState::new()),

    tile_bank: tile::TileBank::new(),

    last_update_nanos: time::precise_time_ns(),
    frame_delta: 0,
  })
}

/// State of the library. Holds state of the systems, like the game renderer.
pub struct LibState<'a> {
  pub renderer : renderer::Renderer<'a>,
  pub input_system: input::InputSystem,
  pub physics_system: physics::PhysicsSystem,
  pub display: glium::backend::glutin_backend::GlutinFacade,
  pub curr_g_state: Option<state::GameState<'a>>,
  engine_logger: logger::Logger,

  /// Bank of tiles used when rendering.
  tile_bank: tile::TileBank,

  /// System time of the last update in nanoseconds. Performance counter time,
  /// NOT time since UNIX epoch! Don't use for current human time!
  last_update_nanos: u64,
  /// Library update delta in nanoseconds
  frame_delta: u64,
}

impl<'a> LibState<'a> {
  /// Update the counter time and delta in LibState.
  fn update_delta(&mut self) {
    let now = time::precise_time_ns();
    self.frame_delta = now - self.last_update_nanos;
    self.last_update_nanos = now;
  }

  pub fn register_tile(&mut self, tile: tile::Tile) {
    self.tile_bank.register_tile(tile);
  }

  pub fn update(&mut self) {
    self.update_delta();

    if self.curr_g_state.is_some() {
      // Process input
      self.input_system.process_input(self.curr_g_state.as_mut().unwrap(), &self.display);

      // Update physics, remember to convert nano second delta into seconds
      self.physics_system.update_physics(self.curr_g_state.as_mut().unwrap(), 
                                         self.frame_delta as f32 / 1000000000.0);

      // Render
      use glium::Surface;
      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);
      self.renderer.render_game(&self.display, &mut target, self.curr_g_state.as_ref().unwrap(), &self.tile_bank);
      let _ = target.finish();
    }
  }
}
