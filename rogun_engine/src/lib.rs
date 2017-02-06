#[macro_use]
extern crate glium;

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
    display: display,
    engine_logger: logger::Logger::new(),
    curr_g_state: Some(state::GameState::new()),
  })
}

/// State of the library. Holds state of the systems, like the game renderer.
pub struct LibState<'a> {
  pub renderer : renderer::Renderer<'a>,
  pub display: glium::backend::glutin_backend::GlutinFacade,
  pub curr_g_state: Option<state::GameState>,
  engine_logger: logger::Logger,
}

impl<'a> LibState<'a> {
  pub fn update(&mut self) {
    if self.curr_g_state.is_some() {
      let mut target = self.display.draw();
      self.renderer.render_game(&self.display, &mut target, self.curr_g_state.as_ref().unwrap());
      let _ = target.finish();
    }
  }
}
