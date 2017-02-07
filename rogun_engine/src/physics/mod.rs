use state::GameState;

pub struct PhysicsSystem {
}

impl PhysicsSystem {
  pub fn new() -> PhysicsSystem {
    PhysicsSystem {}
  }

  /// Update the game state, apply physics.
  /// dt param is update delta in seconds. Keep this constant for consistent
  /// performance across all systems.
  pub fn update_physics(&self, g_state: &mut GameState, dt: f32) {
    // Add velocities to AABB
    let dt2 = dt*dt;
    for body in &mut g_state.comp_body {
      // Set accel from force
      body.acc.0 = body.force.0 / body.mass;
      body.acc.1 = body.force.1 / body.mass;
      body.clear_force();

      if body.vel.0 != 0.0 || body.vel.1 != 0.0 {
        let aabb = g_state.comp_aabb.get_component_mut(body.entity_id);
        if aabb.is_none() { continue; }
        let aabb = aabb.unwrap();

        // Apply velocity (with euler integration)
        aabb.pos.0 += body.vel.0*dt + body.acc.0/2.0*dt2;
        aabb.pos.1 += body.vel.1*dt + body.acc.1/2.0*dt2;
      }

      // Apply accel to vel
      body.vel.0 += body.acc.0*dt;
      body.vel.1 += body.acc.1*dt;

      // Check if at max speed, and cap
      if body.vel.len2() > body.max_speed.powi(2) {
        body.vel.nor().scale(body.max_speed);
      }
    }
  }
}
