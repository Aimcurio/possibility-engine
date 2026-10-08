use possibility_core::{ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GravityDevice {
    pub device_id: ObjectId,
    pub is_active: bool,
    pub center_pos: Vec3,
    pub field_radius: f32,
    pub field_intensity: f32,
}

impl GravityDevice {
    pub fn new(
        device_id: ObjectId,
        center_pos: Vec3,
        field_radius: f32,
        field_intensity: f32,
    ) -> Self {
        Self {
            device_id,
            is_active: false,
            center_pos,
            field_radius,
            field_intensity,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn calculate_field_acceleration(&self, target_pos: Vec3) -> Vec3 {
        if !self.is_active {
            return Vec3::ZERO;
        }

        let delta = self.center_pos - target_pos;
        let dist = delta.length();

        if dist > self.field_radius || dist < 0.05 {
            return Vec3::ZERO;
        }

        let dir = delta.normalize();
        // Bounded radial force model: F = I / (1 + r^2)
        let accel_mag = self.field_intensity / (1.0 + dist * dist);
        dir * accel_mag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_device_lifecycle() {
        let center = Vec3::new(5.0, 1.0, 5.0);
        let mut dev = GravityDevice::new(ObjectId(99), center, 6.0, 50.0);

        let test_obj_pos = Vec3::new(4.0, 1.0, 5.0); // 1.0m away along -X

        // 1. Initially inactive -> zero acceleration
        assert_eq!(dev.calculate_field_acceleration(test_obj_pos), Vec3::ZERO);

        // 2. Active -> attracts toward center
        dev.set_active(true);
        let accel = dev.calculate_field_acceleration(test_obj_pos);
        assert!(accel.x > 0.0, "acceleration must pull toward center x");
        assert_eq!(accel.y, 0.0);
        assert_eq!(accel.z, 0.0);

        // Expected magnitude: 50.0 / (1 + 1^2) = 25.0
        assert!((accel.length() - 25.0).abs() < 1e-4);

        // 3. Beyond radius -> zero acceleration
        let far_pos = Vec3::new(20.0, 1.0, 5.0);
        assert_eq!(dev.calculate_field_acceleration(far_pos), Vec3::ZERO);

        // 4. Deactivate -> terminates field immediately
        dev.set_active(false);
        assert_eq!(dev.calculate_field_acceleration(test_obj_pos), Vec3::ZERO);
    }
}
