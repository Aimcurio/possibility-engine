use crate::goals::{AvatarGoal, TaskStep};
use crate::perception::PerceptionSnapshot;
use possibility_core::{AvatarId, SimCommand, Vec3};
use possibility_sim::{OBJECT_BOWL, OBJECT_CEREAL_BOX, OBJECT_GRAVITY_DEVICE, OBJECT_MILK_CARTON};

#[derive(Debug, Default, Clone)]
pub struct DeterministicPlanner;

impl DeterministicPlanner {
    pub fn new() -> Self {
        Self
    }

    pub fn plan_next_step(&self, goal: &AvatarGoal, snapshot: &PerceptionSnapshot) -> TaskStep {
        match goal {
            AvatarGoal::Idle => TaskStep::Complete,

            AvatarGoal::PrepareBowlOfCereal => {
                let bowl = snapshot.find_object(OBJECT_BOWL);
                let bowl_cereal = bowl.and_then(|b| b.cereal_grams).unwrap_or(0.0);
                let bowl_milk = bowl.and_then(|b| b.milk_ml).unwrap_or(0.0);

                // Check if final condition is already met
                if bowl_cereal >= 25.0 && bowl_milk >= 50.0 {
                    // Release any leftover held containers cleanly
                    if snapshot.is_holding(OBJECT_CEREAL_BOX) {
                        return TaskStep::Place(OBJECT_CEREAL_BOX, Vec3::new(-1.8, 0.85, 3.0));
                    }
                    if snapshot.is_holding(OBJECT_MILK_CARTON) {
                        return TaskStep::Place(OBJECT_MILK_CARTON, Vec3::new(-0.2, 0.85, 3.0));
                    }
                    return TaskStep::Complete;
                }

                // Phase 1: Cereal dispensing
                if bowl_cereal < 25.0 {
                    if snapshot.is_holding(OBJECT_CEREAL_BOX) {
                        // Near bowl?
                        let dist_to_bowl = snapshot.distance_to(OBJECT_BOWL).unwrap_or(99.0);
                        if dist_to_bowl > 2.2 {
                            return TaskStep::NavigateTo(Vec3::new(-1.0, 0.0, 2.0));
                        } else {
                            return TaskStep::DispenseCereal {
                                source: OBJECT_CEREAL_BOX,
                                target: OBJECT_BOWL,
                                amount_g: 45.0,
                            };
                        }
                    } else if snapshot.is_holding(OBJECT_MILK_CARTON) {
                        // Put down milk to pick up cereal
                        return TaskStep::Place(OBJECT_MILK_CARTON, Vec3::new(-0.2, 0.85, 3.0));
                    } else {
                        // Need to navigate to cereal box and pick it up
                        let dist_to_cereal =
                            snapshot.distance_to(OBJECT_CEREAL_BOX).unwrap_or(99.0);
                        if dist_to_cereal > 2.2 {
                            return TaskStep::NavigateTo(Vec3::new(-1.8, 0.0, 2.0));
                        } else {
                            return TaskStep::PickUp(OBJECT_CEREAL_BOX);
                        }
                    }
                }

                // Phase 2: Milk pouring (cereal >= 25.0, milk < 50.0)
                if snapshot.is_holding(OBJECT_CEREAL_BOX) {
                    // Place down cereal box first
                    return TaskStep::Place(OBJECT_CEREAL_BOX, Vec3::new(-1.8, 0.85, 3.0));
                }

                if snapshot.is_holding(OBJECT_MILK_CARTON) {
                    let dist_to_bowl = snapshot.distance_to(OBJECT_BOWL).unwrap_or(99.0);
                    if dist_to_bowl > 2.2 {
                        TaskStep::NavigateTo(Vec3::new(-1.0, 0.0, 2.0))
                    } else {
                        TaskStep::PourMilk {
                            source: OBJECT_MILK_CARTON,
                            target: OBJECT_BOWL,
                            amount_ml: 120.0,
                        }
                    }
                } else {
                    let dist_to_milk = snapshot.distance_to(OBJECT_MILK_CARTON).unwrap_or(99.0);
                    if dist_to_milk > 2.2 {
                        TaskStep::NavigateTo(Vec3::new(-0.2, 0.0, 2.0))
                    } else {
                        TaskStep::PickUp(OBJECT_MILK_CARTON)
                    }
                }
            }

            AvatarGoal::ActivateGravityExperiment => {
                if snapshot.is_gravity_active() {
                    return TaskStep::Complete;
                }
                let terminal_pos = Vec3::new(4.5, 0.0, 1.5);
                let dist = snapshot.avatar_pos.distance(terminal_pos);
                if dist > 2.0 {
                    TaskStep::NavigateTo(terminal_pos)
                } else {
                    TaskStep::SetGravityDevice {
                        device_id: OBJECT_GRAVITY_DEVICE,
                        active: true,
                    }
                }
            }

            AvatarGoal::DeactivateGravityExperiment => {
                if !snapshot.is_gravity_active() {
                    return TaskStep::Complete;
                }
                let terminal_pos = Vec3::new(4.5, 0.0, 1.5);
                let dist = snapshot.avatar_pos.distance(terminal_pos);
                if dist > 2.0 {
                    TaskStep::NavigateTo(terminal_pos)
                } else {
                    TaskStep::SetGravityDevice {
                        device_id: OBJECT_GRAVITY_DEVICE,
                        active: false,
                    }
                }
            }

            AvatarGoal::MoveTo(pos) => {
                if snapshot.avatar_pos.distance(*pos) < 0.2 {
                    TaskStep::Complete
                } else {
                    TaskStep::NavigateTo(*pos)
                }
            }

            AvatarGoal::InspectObject(obj_id) => {
                let dist = snapshot.distance_to(*obj_id).unwrap_or(99.0);
                if dist < 2.0 {
                    TaskStep::Complete
                } else if let Some(target) = snapshot.find_object(*obj_id) {
                    let approach = target.position + Vec3::new(0.0, 0.0, -1.5);
                    TaskStep::NavigateTo(approach)
                } else {
                    TaskStep::Failed(format!("Object {:?} not found", obj_id))
                }
            }

            AvatarGoal::ResetLab => TaskStep::Complete,
        }
    }

    pub fn step_to_command(
        &self,
        avatar_id: AvatarId,
        step: &TaskStep,
        speed: f32,
    ) -> Option<SimCommand> {
        match step {
            TaskStep::NavigateTo(target_pos) => Some(SimCommand::MoveAvatar {
                avatar_id,
                target_pos: *target_pos,
                speed,
            }),
            TaskStep::PickUp(object_id) => Some(SimCommand::PickUpObject {
                avatar_id,
                object_id: *object_id,
            }),
            TaskStep::Place(object_id, position) => Some(SimCommand::PlaceObject {
                avatar_id,
                object_id: *object_id,
                position: *position,
            }),
            TaskStep::DispenseCereal {
                source,
                target,
                amount_g,
            } => Some(SimCommand::DispenseCereal {
                avatar_id,
                source: *source,
                target: *target,
                amount_g: *amount_g,
            }),
            TaskStep::PourMilk {
                source,
                target,
                amount_ml,
            } => Some(SimCommand::PourLiquid {
                avatar_id,
                source: *source,
                target: *target,
                amount_ml: *amount_ml,
            }),
            TaskStep::SetGravityDevice { device_id, active } => {
                Some(SimCommand::SetGravityDeviceState {
                    avatar_id,
                    device_id: *device_id,
                    active: *active,
                })
            }
            TaskStep::Complete | TaskStep::Failed(_) => None,
        }
    }
}
