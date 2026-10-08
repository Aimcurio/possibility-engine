pub mod avatar;
pub mod goals;
pub mod nlp;
pub mod perception;
pub mod planner;

pub use avatar::AvatarAttributes;
pub use goals::{AvatarGoal, TaskStep};
pub use nlp::NaturalLanguageInterpreter;
pub use perception::{PerceivedObject, PerceptionSnapshot};
pub use planner::DeterministicPlanner;

#[cfg(test)]
mod tests {
    use super::*;
    use possibility_core::AvatarId;
    use possibility_sim::{SimWorld, OBJECT_BOWL, OBJECT_GRAVITY_DEVICE};

    #[test]
    fn test_scenario_a_autonomous_planner_loop() {
        let mut world = SimWorld::new_laboratory();
        let planner = DeterministicPlanner::new();
        let avatar_id = AvatarId(0);
        let goal = AvatarGoal::PrepareBowlOfCereal;

        let mut step_count = 0;
        let max_steps = 25;

        loop {
            step_count += 1;
            assert!(
                step_count <= max_steps,
                "Planner exceeded max steps for Scenario A"
            );

            let snapshot = PerceptionSnapshot::capture(&world, avatar_id).expect("snapshot");
            let step = planner.plan_next_step(&goal, &snapshot);

            if matches!(step, TaskStep::Complete) {
                break;
            }

            if let Some(cmd) = planner.step_to_command(avatar_id, &step, 3.5) {
                world.apply_command(cmd).expect("command execution");
            }
            world.step(0.1);
        }

        let final_snapshot = PerceptionSnapshot::capture(&world, avatar_id).unwrap();
        assert!(final_snapshot.is_cereal_ready());
        let bowl = final_snapshot.find_object(OBJECT_BOWL).unwrap();
        assert!(bowl.cereal_grams.unwrap() >= 25.0);
        assert!(bowl.milk_ml.unwrap() >= 50.0);
    }

    #[test]
    fn test_scenario_b_autonomous_gravity_loop() {
        let mut world = SimWorld::new_laboratory();
        let planner = DeterministicPlanner::new();
        let avatar_id = AvatarId(1);
        let goal = AvatarGoal::ActivateGravityExperiment;

        let mut step_count = 0;
        loop {
            step_count += 1;
            assert!(step_count <= 10, "Exceeded steps for gravity activation");

            let snapshot = PerceptionSnapshot::capture(&world, avatar_id).expect("snapshot");
            let step = planner.plan_next_step(&goal, &snapshot);

            if matches!(step, TaskStep::Complete) {
                break;
            }

            if let Some(cmd) = planner.step_to_command(avatar_id, &step, 3.5) {
                world.apply_command(cmd).expect("command execution");
            }
            world.step(0.1);
        }

        let final_snapshot = PerceptionSnapshot::capture(&world, avatar_id).unwrap();
        assert!(final_snapshot.is_gravity_active());
        let dev = final_snapshot.find_object(OBJECT_GRAVITY_DEVICE).unwrap();
        assert_eq!(dev.is_active_device, Some(true));
    }
}
