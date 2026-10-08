use possibility_core::{ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AvatarGoal {
    Idle,
    PrepareBowlOfCereal,
    ActivateGravityExperiment,
    DeactivateGravityExperiment,
    InspectObject(ObjectId),
    MoveTo(Vec3),
    ResetLab,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStep {
    NavigateTo(Vec3),
    PickUp(ObjectId),
    Place(ObjectId, Vec3),
    DispenseCereal {
        source: ObjectId,
        target: ObjectId,
        amount_g: f32,
    },
    PourMilk {
        source: ObjectId,
        target: ObjectId,
        amount_ml: f32,
    },
    SetGravityDevice {
        device_id: ObjectId,
        active: bool,
    },
    Complete,
    Failed(String),
}
