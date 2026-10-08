use bevy::prelude::*;
use possibility_cognition::AvatarGoal;
use possibility_control::{ControlMode, InterruptionPolicy};
use possibility_core::{AvatarId, SimCommand, Vec3 as CoreVec3};
use possibility_sim::{OBJECT_BOWL, OBJECT_CEREAL_BOX, OBJECT_MILK_CARTON};

use crate::app::{ArbitratorResource, SimWorldResource};

#[derive(Resource)]
pub struct SelectedAvatar(pub AvatarId);

impl Default for SelectedAvatar {
    fn default() -> Self {
        Self(AvatarId(2)) // Default selected: Gamma (Human)
    }
}

pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedAvatar>,
    mut world_res: ResMut<SimWorldResource>,
    mut arb_res: ResMut<ArbitratorResource>,
) {
    // 1. Avatar Selection
    if keyboard.just_pressed(KeyCode::Digit1) {
        selected.0 = AvatarId(0);
        info!("Selected Avatar 0: Alpha");
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        selected.0 = AvatarId(1);
        info!("Selected Avatar 1: Beta");
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        selected.0 = AvatarId(2);
        info!("Selected Avatar 2: Gamma");
    }

    let active_id = selected.0;

    // 2. Control Mode Selection
    if keyboard.just_pressed(KeyCode::KeyH) {
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::Human,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!("Avatar {:?} switched to Human control", active_id);
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::AutonomousAI,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!("Avatar {:?} switched to Autonomous AI", active_id);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::DirectedAI,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!("Avatar {:?} switched to Directed AI", active_id);
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::Copilot,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!("Avatar {:?} switched to Copilot", active_id);
    }

    // 3. Scenario Triggers
    if keyboard.just_pressed(KeyCode::F1) {
        let _ = arb_res
            .0
            .assign_goal(active_id, AvatarGoal::PrepareBowlOfCereal);
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::DirectedAI,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!(
            "Triggered Scenario A (Prepare Bowl of Cereal) on Avatar {:?}",
            active_id
        );
    } else if keyboard.just_pressed(KeyCode::F2) {
        let _ = arb_res
            .0
            .assign_goal(active_id, AvatarGoal::ActivateGravityExperiment);
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::DirectedAI,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
        info!(
            "Triggered Scenario B (Activate Gravity Field) on Avatar {:?}",
            active_id
        );
    } else if keyboard.just_pressed(KeyCode::F3) {
        info!("Running Scenario C (Control Handoff demonstration)");
        // Trigger automated handoff cycle
        let _ = arb_res.0.switch_mode(
            active_id,
            ControlMode::Human,
            &world_res.0,
            InterruptionPolicy::SuspendAndPreserve,
        );
    }

    // 4. World Reset
    if keyboard.just_pressed(KeyCode::KeyR) {
        world_res.0.reset();
        arb_res.0 = possibility_control::ControlArbitrator::new();
        info!("World & Simulation reset to initial laboratory state.");
    }

    // 5. Direct Human WASD Movement
    if let Some(mode) = arb_res.0.get_mode(active_id) {
        if mode == ControlMode::Human || mode == ControlMode::Copilot {
            let mut move_dir = CoreVec3::ZERO;
            if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
                move_dir.z -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
                move_dir.z += 1.0;
            }
            if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
                move_dir.x -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
                move_dir.x += 1.0;
            }

            if move_dir.length_squared() > 0.0 {
                let dir = move_dir.normalize();
                if let Some(av) = world_res.0.get_avatar(active_id) {
                    let new_pos = av.position + dir * 0.15;
                    let _ = world_res.0.apply_command(SimCommand::MoveAvatar {
                        avatar_id: active_id,
                        target_pos: new_pos,
                        speed: 3.5,
                    });
                }
            }

            // Spacebar interaction
            if keyboard.just_pressed(KeyCode::Space) {
                if let Some(av) = world_res.0.get_avatar(active_id) {
                    if let Some(held) = av.grasp.held_object {
                        // Place held object
                        let place_pos = av.position + CoreVec3::new(0.0, 0.85, 1.0);
                        let _ = world_res.0.apply_command(SimCommand::PlaceObject {
                            avatar_id: active_id,
                            object_id: held,
                            position: place_pos,
                        });
                    } else {
                        // Attempt to pick up closest grabbable object
                        for &cand in &[OBJECT_CEREAL_BOX, OBJECT_MILK_CARTON, OBJECT_BOWL] {
                            if let Some(obj) = world_res.0.get_object(cand) {
                                if av.position.distance(obj.body.position) < 2.5 {
                                    let _ = world_res.0.apply_command(SimCommand::PickUpObject {
                                        avatar_id: active_id,
                                        object_id: cand,
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
