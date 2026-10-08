use crate::types::{AvatarId, ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimCommand {
    MoveAvatar {
        avatar_id: AvatarId,
        target_pos: Vec3,
        speed: f32,
    },
    PickUpObject {
        avatar_id: AvatarId,
        object_id: ObjectId,
    },
    PlaceObject {
        avatar_id: AvatarId,
        object_id: ObjectId,
        position: Vec3,
    },
    PourLiquid {
        avatar_id: AvatarId,
        source: ObjectId,
        target: ObjectId,
        amount_ml: f32,
    },
    DispenseCereal {
        avatar_id: AvatarId,
        source: ObjectId,
        target: ObjectId,
        amount_g: f32,
    },
    SetGravityDeviceState {
        avatar_id: AvatarId,
        device_id: ObjectId,
        active: bool,
    },
    ResetSimulation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommandValidationError {
    AvatarNotFound(AvatarId),
    ObjectNotFound(ObjectId),
    ObjectAlreadyHeld {
        object_id: ObjectId,
        held_by: AvatarId,
    },
    ObjectNotHeld {
        avatar_id: AvatarId,
        object_id: ObjectId,
    },
    TargetOutOfReach {
        distance: f32,
        max_reach: f32,
    },
    SourceNotOpen(ObjectId),
    InsufficientContents {
        object_id: ObjectId,
        requested: f32,
        available: f32,
    },
    ContainerFull {
        object_id: ObjectId,
        capacity: f32,
        current: f32,
    },
    DeviceNotRecognized(ObjectId),
}
