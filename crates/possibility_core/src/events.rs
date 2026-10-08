use crate::invariants::InvariantViolation;
use crate::types::{AvatarId, ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimEvent {
    AvatarMoved {
        avatar_id: AvatarId,
        position: Vec3,
    },
    ObjectPickedUp {
        avatar_id: AvatarId,
        object_id: ObjectId,
    },
    ObjectPlaced {
        avatar_id: AvatarId,
        object_id: ObjectId,
        position: Vec3,
    },
    ContentsTransferred {
        source: ObjectId,
        target: ObjectId,
        cereal_amount_g: f32,
        milk_amount_ml: f32,
    },
    GravityDeviceStateChanged {
        device_id: ObjectId,
        active: bool,
        field_radius: f32,
    },
    SimulationReset,
    InvariantViolationDetected(InvariantViolation),
}
