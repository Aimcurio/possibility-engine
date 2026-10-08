use possibility_core::{AvatarId, ObjectId, Vec3};
use possibility_sim::{SimWorld, OBJECT_BOWL, OBJECT_GRAVITY_DEVICE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceivedObject {
    pub id: ObjectId,
    pub name: String,
    pub position: Vec3,
    pub distance_to_avatar: f32,
    pub is_grabbable: bool,
    pub is_device: bool,
    pub cereal_grams: Option<f32>,
    pub milk_ml: Option<f32>,
    pub is_active_device: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptionSnapshot {
    pub avatar_id: AvatarId,
    pub avatar_pos: Vec3,
    pub held_object: Option<ObjectId>,
    pub objects: Vec<PerceivedObject>,
}

impl PerceptionSnapshot {
    pub fn capture(world: &SimWorld, avatar_id: AvatarId) -> Option<Self> {
        let avatar = world.get_avatar(avatar_id)?;
        let avatar_pos = avatar.position;
        let held = avatar.grasp.held_object;

        let mut objects = Vec::new();
        for (obj_id, obj_rec) in &world.objects {
            let dist = avatar_pos.distance(obj_rec.body.position);
            let container = world.get_container(*obj_id);
            let device = world.get_gravity_device(*obj_id);

            objects.push(PerceivedObject {
                id: *obj_id,
                name: obj_rec.name.clone(),
                position: obj_rec.body.position,
                distance_to_avatar: dist,
                is_grabbable: obj_rec.is_grabbable,
                is_device: obj_rec.is_device,
                cereal_grams: container.map(|c| c.cereal_grams),
                milk_ml: container.map(|c| c.milk_ml),
                is_active_device: device.map(|d| d.is_active),
            });
        }

        Some(Self {
            avatar_id,
            avatar_pos,
            held_object: held,
            objects,
        })
    }

    pub fn find_object(&self, id: ObjectId) -> Option<&PerceivedObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    pub fn is_holding(&self, id: ObjectId) -> bool {
        self.held_object == Some(id)
    }

    pub fn distance_to(&self, id: ObjectId) -> Option<f32> {
        self.find_object(id).map(|o| o.distance_to_avatar)
    }

    pub fn is_cereal_ready(&self) -> bool {
        if let Some(bowl) = self.find_object(OBJECT_BOWL) {
            let c = bowl.cereal_grams.unwrap_or(0.0);
            let m = bowl.milk_ml.unwrap_or(0.0);
            c >= 25.0 && m >= 50.0
        } else {
            false
        }
    }

    pub fn is_gravity_active(&self) -> bool {
        if let Some(dev) = self.find_object(OBJECT_GRAVITY_DEVICE) {
            dev.is_active_device.unwrap_or(false)
        } else {
            false
        }
    }
}
