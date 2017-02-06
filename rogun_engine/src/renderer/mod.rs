use glium;
use glium::uniforms::{UniformsStorage, EmptyUniforms};
use glium::backend::glutin_backend::GlutinFacade;
use state::GameState;

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 4],
}

implement_vertex!(Vertex, position, color);

pub struct Renderer<'a> {
  program: glium::Program,
  uniforms: UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>,
  view_w: u32,
  view_h: u32,
}

impl<'a> Renderer<'a> {
  pub fn new(display: &GlutinFacade, w: u32, h: u32) -> Renderer<'a> {
    // Vertex shader
    let vert_src = r#"
      #version 100
      attribute vec2 position;
      attribute vec4 color;

      varying vec4 v_color;

      uniform mat4 proj_mat;

      void main() {
          v_color = color;
          gl_Position = proj_mat * vec4(position, 0.0, 1.0);
      }
    "#;

    // Fragment shader
    let frag_src = r#"
      #version 100
      precision mediump float; // Float precision to medium

      varying vec4 v_color;

      void main() {
        gl_FragColor = v_color;
      }
    "#;

    let uniforms = UniformsStorage::new(
      "proj_mat",
      // Orthographic proj mat:
      // glOrtho(0, w, h, 0, -1, 1);
      [[2.0/w as f32, 0.0,           0.0, -0.0],
       [0.0,         -2.0/h as f32,  0.0,  0.0],
       [0.0,          0.0,          -1.0,  0.0],
       [-1.0,          1.0,           0.0,  1.0]]);

    Renderer { 
      uniforms: uniforms,
      view_w: w, view_h: h,
      program: glium::Program::from_source(display,
                                           vert_src, 
                                           frag_src, 
                                           None).unwrap(),
    }
  }

  pub fn render_game(&self, display: &GlutinFacade, 
                     target: &mut glium::Frame, 
                     g_state: &GameState) {
    use glium::Surface;
    let mut data = Vec::new();

    /// Add debug draws to the VBO
    for c in &g_state.comp_debug_draw {
      /// Find AABB
      let aabb = g_state.comp_aabb.get_component(c.entity_id);
      if aabb.is_none() { continue; }
      let aabb = aabb.unwrap();

      /// Push data to VBO
      data.push(Vertex {
        position: [aabb.pos.0, aabb.pos.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
      data.push(Vertex {
        position: [aabb.pos.0 + aabb.size.0, aabb.pos.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
      data.push(Vertex {
        position: [aabb.pos.0 + aabb.size.0, aabb.pos.1 + aabb.size.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
      data.push(Vertex {
        position: [aabb.pos.0, aabb.pos.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
      data.push(Vertex {
        position: [aabb.pos.0, aabb.pos.1 + aabb.size.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
      data.push(Vertex {
        position: [aabb.pos.0 + aabb.size.0, aabb.pos.1 + aabb.size.1],
        color: [c.color.r, c.color.g, c.color.b, 1.0],
      });
    }

    let vbo = glium::VertexBuffer::new(display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut draw_params = glium::draw_parameters::DrawParameters::default();
    target.draw(&vbo, indices, &self.program, &self.uniforms, &draw_params).unwrap();
  }
}
