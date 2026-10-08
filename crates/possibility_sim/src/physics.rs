use possibility_core::{Vec3, WorldBounds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub is_static: bool,
    pub damping: f32,
    pub initial_position: Vec3,
    pub initial_velocity: Vec3,
}

impl RigidBody {
    pub fn new(position: Vec3, mass: f32, is_static: bool) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            mass,
            is_static,
            damping: 0.15,
            initial_position: position,
            initial_velocity: Vec3::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.position = self.initial_position;
        self.velocity = self.initial_velocity;
    }

    pub fn step(&mut self, external_accel: Vec3, dt: f32, bounds: &WorldBounds) -> Option<f32> {
        if self.is_static {
            return None;
        }

        // Apply external acceleration (gravity field, etc.)
        self.velocity = self.velocity + external_accel * dt;

        // Apply linear damping
        let damp_factor = (1.0 - self.damping * dt).clamp(0.0, 1.0);
        self.velocity = self.velocity * damp_factor;

        // Velocity clamping to invariant max speed
        let mut excessive_speed = None;
        let speed = self.velocity.length();
        if speed > bounds.max_velocity {
            excessive_speed = Some(speed);
            self.velocity = self.velocity.normalize() * bounds.max_velocity;
        }

        // Integrate position
        self.position = self.position + self.velocity * dt;

        // Floor collision response (lab floor at y = 0.0 or table surface)
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            if self.velocity.y < 0.0 {
                self.velocity.y = -self.velocity.y * 0.2;
                if self.velocity.y.abs() < 0.1 {
                    self.velocity.y = 0.0;
                }
            }
        }

        // World bounds clamping
        self.position.x = self.position.x.clamp(bounds.min.x, bounds.max.x);
        self.position.y = self.position.y.clamp(bounds.min.y, bounds.max.y);
        self.position.z = self.position.z.clamp(bounds.min.z, bounds.max.z);

        excessive_speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigidbody_kinematics() {
        let bounds = WorldBounds::default();
        let mut rb = RigidBody::new(Vec3::new(0.0, 10.0, 0.0), 2.0, false);

        // Apply external acceleration along X
        let accel = Vec3::new(5.0, 0.0, 0.0);
        rb.step(accel, 0.1, &bounds);

        assert!(rb.velocity.x > 0.0);
        assert!(rb.position.x > 0.0);
    }

    #[test]
    fn test_rigidbody_reset() {
        let bounds = WorldBounds::default();
        let start = Vec3::new(2.0, 3.0, 4.0);
        let mut rb = RigidBody::new(start, 1.0, false);

        rb.step(Vec3::new(10.0, 0.0, 0.0), 1.0, &bounds);
        assert_ne!(rb.position, start);

        rb.reset();
        assert_eq!(rb.position, start);
        assert_eq!(rb.velocity, Vec3::ZERO);
    }
}
