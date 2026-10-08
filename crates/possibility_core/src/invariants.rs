use crate::types::{ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvariantViolation {
    NegativeVolume {
        object_id: ObjectId,
        volume: f32,
    },
    VolumeExceedsCapacity {
        object_id: ObjectId,
        volume: f32,
        capacity: f32,
    },
    PositionOutOfBounds {
        object_id: ObjectId,
        position: Vec3,
        bounds_min: Vec3,
        bounds_max: Vec3,
    },
    ExcessiveVelocity {
        object_id: ObjectId,
        velocity: Vec3,
        max_speed: f32,
    },
    NegativeMass {
        object_id: ObjectId,
        mass: f32,
    },
    MassConservationViolation {
        expected_total: f32,
        actual_total: f32,
        delta: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorldBounds {
    pub min: Vec3,
    pub max: Vec3,
    pub max_velocity: f32,
}

impl Default for WorldBounds {
    fn default() -> Self {
        Self {
            min: Vec3::new(-50.0, -2.0, -50.0),
            max: Vec3::new(50.0, 50.0, 50.0),
            max_velocity: 50.0,
        }
    }
}

impl WorldBounds {
    pub fn validate_position(
        &self,
        object_id: ObjectId,
        pos: Vec3,
    ) -> Result<(), InvariantViolation> {
        if pos.x < self.min.x
            || pos.x > self.max.x
            || pos.y < self.min.y
            || pos.y > self.max.y
            || pos.z < self.min.z
            || pos.z > self.max.z
        {
            Err(InvariantViolation::PositionOutOfBounds {
                object_id,
                position: pos,
                bounds_min: self.min,
                bounds_max: self.max,
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_velocity(
        &self,
        object_id: ObjectId,
        vel: Vec3,
    ) -> Result<(), InvariantViolation> {
        let speed = vel.length();
        if speed > self.max_velocity {
            Err(InvariantViolation::ExcessiveVelocity {
                object_id,
                velocity: vel,
                max_speed: self.max_velocity,
            })
        } else {
            Ok(())
        }
    }
}
