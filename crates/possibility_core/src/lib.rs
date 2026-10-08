pub mod commands;
pub mod events;
pub mod invariants;
pub mod types;

pub use commands::{CommandValidationError, SimCommand};
pub use events::SimEvent;
pub use invariants::{InvariantViolation, WorldBounds};
pub use types::{Aabb, AvatarId, EntityId, MaterialType, ObjectId, PhysicalProperties, Vec3};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));

        let diff = v2 - v1;
        assert_eq!(diff, Vec3::new(3.0, 3.0, 3.0));

        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vec3::new(2.0, 4.0, 6.0));

        let dist = v1.distance(v2);
        assert!((dist - 27.0_f32.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn test_aabb_intersection() {
        let box1 = Aabb::from_center_half_extents(Vec3::ZERO, Vec3::ONE);
        let box2 = Aabb::from_center_half_extents(Vec3::new(1.5, 0.0, 0.0), Vec3::ONE);
        assert!(box1.intersects(&box2));

        let box3 = Aabb::from_center_half_extents(Vec3::new(3.0, 0.0, 0.0), Vec3::ONE);
        assert!(!box1.intersects(&box3));
    }

    #[test]
    fn test_world_bounds_validation() {
        let bounds = WorldBounds::default();
        let valid_pos = Vec3::new(10.0, 1.0, -10.0);
        assert!(bounds.validate_position(ObjectId(1), valid_pos).is_ok());

        let invalid_pos = Vec3::new(100.0, 0.0, 0.0);
        assert!(bounds.validate_position(ObjectId(1), invalid_pos).is_err());
    }
}
