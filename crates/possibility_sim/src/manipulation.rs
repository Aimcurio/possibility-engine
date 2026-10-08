use possibility_core::{ObjectId, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraspSocket {
    pub held_object: Option<ObjectId>,
    pub local_offset: Vec3,
}

impl Default for GraspSocket {
    fn default() -> Self {
        Self {
            held_object: None,
            local_offset: Vec3::new(0.0, 0.9, 0.4),
        }
    }
}

impl GraspSocket {
    pub fn is_holding(&self) -> bool {
        self.held_object.is_some()
    }

    pub fn attach(&mut self, object_id: ObjectId) -> Result<(), String> {
        if let Some(existing) = self.held_object {
            return Err(format!("Socket already holds object {:?}", existing));
        }
        self.held_object = Some(object_id);
        Ok(())
    }

    pub fn detach(&mut self) -> Option<ObjectId> {
        self.held_object.take()
    }

    pub fn calculate_held_world_pos(&self, avatar_pos: Vec3) -> Vec3 {
        avatar_pos + self.local_offset
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssemblyComponent {
    pub parent_object: ObjectId,
    pub child_object: ObjectId,
    pub relative_offset: Vec3,
}
