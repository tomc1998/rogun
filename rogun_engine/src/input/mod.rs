use state::GameState;
use glium::glutin::{Event, ElementState, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;
use common::vec::Vec2f32;

/// Struct representing a key currently pressed on the keyboard.
#[derive(Copy, Clone)]
pub struct KeyDown {
  pub key_code : VirtualKeyCode,

  /// True if this has just been pressed. Every time process_input is called on
  /// the InputSystem, all just_pressed values are set to false. So this should
  /// be true for 1 frame.
  pub just_pressed : bool,
}

pub struct InputSystem {
  /// List of keys currently pressed. Maybe a bit slow?
  keys_down : Vec<KeyDown>,
}

impl InputSystem {
  pub fn new() -> InputSystem { 
    InputSystem { keys_down: Vec::with_capacity(6) }
  }

  /// Checks if a key is down. Returns the 'KeyDown' struct if the key is down,
  /// or None if not.
  /// Just linear searches through the list, shouldn't pose too much of an
  /// issue as the list will likely never be longer than ~6.
  pub fn is_key_down(&self, key_code: VirtualKeyCode) -> Option<KeyDown> {
    for k in &self.keys_down {
      if k.key_code == key_code {
        return Some(*k);
      }
    }
    return None;
  }

  /// Removes a key_down entry from the list of keys_down
  fn remove_key_down(&mut self, key_code: VirtualKeyCode) {
    for ii in 0..self.keys_down.len() {
      if self.keys_down[ii].key_code == key_code {
        self.keys_down.remove(ii);
        break;
      }
    }
  }

  /// Updates keys_down and other input system state
  fn update_input_state(&mut self, display: &GlutinFacade) {
    // Loop over all current KeyDowns and set just_pressed to false.
    for k in &mut self.keys_down {
      k.just_pressed = false;
    }

    'Outer: for e in display.poll_events() {
      match e {
        Event::KeyboardInput(state, _, key_code) => {
          // No idea why the key code would be none, not documented in glium
          if key_code.is_none() { continue 'Outer; }
          let key_code = key_code.unwrap();
          if state == ElementState::Pressed {
            // Add to list
            if self.is_key_down(key_code).is_none() {
              self.keys_down.push(KeyDown {
                key_code: key_code,
                just_pressed: true,
              });
            }
          }
          else if state == ElementState::Released {
            // Remove from list
            self.remove_key_down(key_code);
          }
        }
        _ => continue 'Outer,
      }
    }
  }

  pub fn process_input(&mut self, g_state: &mut GameState, display: &GlutinFacade) {
    // Process new keyboard input
    self.update_input_state(display);

    // Check l/r controller keys (have they been pressed?
    for c in &g_state.comp_lr_controller {
      let key_l_down = self.is_key_down(c.key_l);
      let key_r_down = self.is_key_down(c.key_r);
      // Check if key_l is down
      if key_r_down.is_some() && key_l_down.is_none() {
        // Find entity body 
        let body = g_state.comp_body.get_component_mut(c.entity_id);
        if body.is_none() { continue; }
        let body = body.unwrap();
        // Don't set acceleration, as this won't stack with other
        // acceleration
        body.apply_force(Vec2f32(c.force, 0.0));
      }
      else if key_l_down.is_some() && key_r_down.is_none() {
        // Find entity body 
        let body = g_state.comp_body.get_component_mut(c.entity_id);
        if body.is_none() { continue; }
        let body = body.unwrap();
        // Don't set acceleration, as this won't stack with other
        // acceleration
        body.apply_force(Vec2f32(-c.force, 0.0));
      }
    }
  }
}
