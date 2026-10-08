use crate::modes::ControlMode;
use possibility_cognition::AvatarGoal;
use possibility_core::{AvatarId, ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandoffRecord {
    pub avatar_id: AvatarId,
    pub previous_mode: ControlMode,
    pub new_mode: ControlMode,
    pub avatar_position: Vec3,
    pub held_object: Option<ObjectId>,
    pub preserved_goal: Option<AvatarGoal>,
    pub timestamp: f32,
    pub state_preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptionPolicy {
    SuspendAndPreserve,
    CancelGoal,
}
