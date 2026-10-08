pub mod arbitrator;
pub mod handoff;
pub mod modes;

pub use arbitrator::{AvatarControlState, ControlArbitrator};
pub use handoff::{HandoffRecord, InterruptionPolicy};
pub use modes::ControlMode;

#[cfg(test)]
mod tests {
    use super::*;
    use possibility_cognition::AvatarGoal;
    use possibility_core::{AvatarId, SimCommand, Vec3};
    use possibility_sim::{SimWorld, OBJECT_CEREAL_BOX};

    #[test]
    fn test_scenario_c_control_handoff_and_resumption() {
        let mut world = SimWorld::new_laboratory();
        let mut arbitrator = ControlArbitrator::new();
        let avatar_id = AvatarId(0);

        // 1. Initial State: Avatar 0 is Directed AI working on PrepareBowlOfCereal
        assert_eq!(
            arbitrator.get_mode(avatar_id),
            Some(ControlMode::DirectedAI)
        );
        assert_eq!(
            arbitrator.get_goal(avatar_id),
            Some(&AvatarGoal::PrepareBowlOfCereal)
        );

        // Run AI for 2 steps (navigates towards counter, picks up cereal box)
        for _ in 0..2 {
            if let Some(cmd) = arbitrator.tick_avatar(avatar_id, &world, None) {
                world.apply_command(cmd).expect("apply cmd");
                world.step(0.1);
            }
        }

        // Verify avatar picked up cereal box
        let avatar_rec = world.get_avatar(avatar_id).unwrap();
        assert_eq!(avatar_rec.grasp.held_object, Some(OBJECT_CEREAL_BOX));
        let pos_before_handoff = avatar_rec.position;

        // 2. Control Handoff: Switch Avatar 0 from Directed AI -> Human
        let handoff_record = arbitrator
            .switch_mode(
                avatar_id,
                ControlMode::Human,
                &world,
                InterruptionPolicy::SuspendAndPreserve,
            )
            .expect("switch to human");

        assert_eq!(handoff_record.previous_mode, ControlMode::DirectedAI);
        assert_eq!(handoff_record.new_mode, ControlMode::Human);
        assert_eq!(handoff_record.held_object, Some(OBJECT_CEREAL_BOX));
        assert!(handoff_record.state_preserved);

        // 3. Human Control: Move avatar manually while holding cereal box
        let human_target = pos_before_handoff + Vec3::new(0.5, 0.0, 0.0);
        let human_move_cmd = SimCommand::MoveAvatar {
            avatar_id,
            target_pos: human_target,
            speed: 2.0,
        };
        let executed_cmd = arbitrator
            .tick_avatar(avatar_id, &world, Some(human_move_cmd.clone()))
            .expect("human cmd");
        world.apply_command(executed_cmd).expect("apply human move");
        world.step(0.1);

        // Verify position moved and cereal box is STILL securely held
        let avatar_after_human = world.get_avatar(avatar_id).unwrap();
        assert_eq!(avatar_after_human.position, human_target);
        assert_eq!(
            avatar_after_human.grasp.held_object,
            Some(OBJECT_CEREAL_BOX)
        );

        // 4. Return Control to AI: Switch back to Directed AI
        let return_record = arbitrator
            .switch_mode(
                avatar_id,
                ControlMode::DirectedAI,
                &world,
                InterruptionPolicy::SuspendAndPreserve,
            )
            .expect("switch back to AI");

        assert_eq!(return_record.previous_mode, ControlMode::Human);
        assert_eq!(return_record.new_mode, ControlMode::DirectedAI);
        assert_eq!(
            arbitrator.get_goal(avatar_id),
            Some(&AvatarGoal::PrepareBowlOfCereal)
        );

        // 5. AI Resumes and completes the remaining steps cleanly
        let mut steps = 0;
        while steps < 20 {
            steps += 1;
            if let Some(cmd) = arbitrator.tick_avatar(avatar_id, &world, None) {
                world.apply_command(cmd).expect("apply resume cmd");
                world.step(0.1);
            } else {
                break;
            }
        }

        // Verify bowl is successfully completed despite the interruption and manual detour!
        let bowl = world.get_container(possibility_sim::OBJECT_BOWL).unwrap();
        assert!(
            bowl.is_prepared_cereal(),
            "Interrupted AI must successfully finish the cereal task!"
        );
    }
}
