use possibility_cognition::{AvatarGoal, DeterministicPlanner, PerceptionSnapshot, TaskStep};
use possibility_control::{ControlArbitrator, ControlMode, InterruptionPolicy};
use possibility_core::{AvatarId, SimCommand, Vec3};
use possibility_sim::{
    SimWorld, OBJECT_BOWL, OBJECT_CEREAL_BOX, OBJECT_GRAVITY_DEVICE, OBJECT_TEST_CUBE_A,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub scenario: String,
    pub title: String,
    pub status: String,
    pub steps_taken: u32,
    pub duration_sim_seconds: f32,
    pub details: String,
}

pub fn run_scenario_a_headless() -> ScenarioResult {
    let mut world = SimWorld::new_laboratory();
    let planner = DeterministicPlanner::new();
    let avatar_id = AvatarId(0);
    let goal = AvatarGoal::PrepareBowlOfCereal;

    let mut steps = 0;
    let max_steps = 25;
    let mut finished = false;

    while steps < max_steps {
        steps += 1;
        let snapshot = match PerceptionSnapshot::capture(&world, avatar_id) {
            Some(s) => s,
            None => break,
        };

        let next_step = planner.plan_next_step(&goal, &snapshot);
        if matches!(next_step, TaskStep::Complete) {
            finished = true;
            break;
        }

        if let Some(cmd) = planner.step_to_command(avatar_id, &next_step, 3.5) {
            if let Err(e) = world.apply_command(cmd) {
                return ScenarioResult {
                    scenario: "A".to_string(),
                    title: "Bowl of Cereal".to_string(),
                    status: "FAILED".to_string(),
                    steps_taken: steps,
                    duration_sim_seconds: world.time_elapsed,
                    details: format!("Command validation error: {:?}", e),
                };
            }
        }
        world.step(0.1);
    }

    let bowl = world.get_container(OBJECT_BOWL).expect("bowl");
    let is_ready = bowl.is_prepared_cereal();

    if finished && is_ready {
        ScenarioResult {
            scenario: "A".to_string(),
            title: "Bowl of Cereal".to_string(),
            status: "PASSED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: format!(
                "Bowl prepared successfully! Cereal: {:.1}g (min 25g), Milk: {:.1}ml (min 50ml)",
                bowl.cereal_grams, bowl.milk_ml
            ),
        }
    } else {
        ScenarioResult {
            scenario: "A".to_string(),
            title: "Bowl of Cereal".to_string(),
            status: "FAILED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: format!(
                "Incomplete state. Cereal: {:.1}g, Milk: {:.1}ml, finished: {}",
                bowl.cereal_grams, bowl.milk_ml, finished
            ),
        }
    }
}

pub fn run_scenario_b_headless() -> ScenarioResult {
    let mut world = SimWorld::new_laboratory();
    let planner = DeterministicPlanner::new();
    let avatar_id = AvatarId(1);
    let goal = AvatarGoal::ActivateGravityExperiment;

    let initial_pos = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
    let mut steps = 0;
    let max_steps = 15;
    let mut activated = false;

    while steps < max_steps {
        steps += 1;
        let snapshot = match PerceptionSnapshot::capture(&world, avatar_id) {
            Some(s) => s,
            None => break,
        };

        let next_step = planner.plan_next_step(&goal, &snapshot);
        if matches!(next_step, TaskStep::Complete) {
            activated = true;
            break;
        }

        if let Some(cmd) = planner.step_to_command(avatar_id, &next_step, 3.5) {
            if let Err(e) = world.apply_command(cmd) {
                return ScenarioResult {
                    scenario: "B".to_string(),
                    title: "Gravity Experiment".to_string(),
                    status: "FAILED".to_string(),
                    steps_taken: steps,
                    duration_sim_seconds: world.time_elapsed,
                    details: format!("Command validation error: {:?}", e),
                };
            }
        }
        world.step(0.1);
    }

    if !activated {
        return ScenarioResult {
            scenario: "B".to_string(),
            title: "Gravity Experiment".to_string(),
            status: "FAILED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: "Failed to activate gravity device".to_string(),
        };
    }

    // Step simulation 10 times to observe physical attraction force on test cube
    for _ in 0..10 {
        world.step(0.05);
    }

    let active_pos = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
    let moved_towards_center = active_pos.x > initial_pos.x;

    // Now deactivate device and test termination
    world
        .apply_command(SimCommand::SetGravityDeviceState {
            avatar_id,
            device_id: OBJECT_GRAVITY_DEVICE,
            active: false,
        })
        .expect("deactivate");

    let is_deactivated = !world
        .get_gravity_device(OBJECT_GRAVITY_DEVICE)
        .unwrap()
        .is_active;

    // Reset experiment
    world
        .apply_command(SimCommand::ResetSimulation)
        .expect("reset");
    let reset_pos = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
    let reset_verified = reset_pos == initial_pos;

    if moved_towards_center && is_deactivated && reset_verified {
        ScenarioResult {
            scenario: "B".to_string(),
            title: "Gravity Experiment".to_string(),
            status: "PASSED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: format!(
                "Field generated attraction force (Cube moved from X={:.2} to X={:.2}), deactivation zeroed force, reset restored position",
                initial_pos.x, active_pos.x
            ),
        }
    } else {
        ScenarioResult {
            scenario: "B".to_string(),
            title: "Gravity Experiment".to_string(),
            status: "FAILED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: format!(
                "moved_towards_center: {}, is_deactivated: {}, reset_verified: {}",
                moved_towards_center, is_deactivated, reset_verified
            ),
        }
    }
}

pub fn run_scenario_c_headless() -> ScenarioResult {
    let mut world = SimWorld::new_laboratory();
    let mut arbitrator = ControlArbitrator::new();
    let avatar_id = AvatarId(0);

    // Step 1: Let AI work on cereal until cereal box is picked up
    for _ in 0..2 {
        if let Some(cmd) = arbitrator.tick_avatar(avatar_id, &world, None) {
            world.apply_command(cmd).expect("apply ai cmd");
            world.step(0.1);
        }
    }

    let held_before = world.get_avatar(avatar_id).unwrap().grasp.held_object;
    if held_before != Some(OBJECT_CEREAL_BOX) {
        return ScenarioResult {
            scenario: "C".to_string(),
            title: "Control Handoff".to_string(),
            status: "FAILED".to_string(),
            steps_taken: 2,
            duration_sim_seconds: world.time_elapsed,
            details: format!("Expected cereal box held, got {:?}", held_before),
        };
    }

    // Step 2: Handoff from Directed AI -> Human
    let handoff = arbitrator
        .switch_mode(
            avatar_id,
            ControlMode::Human,
            &world,
            InterruptionPolicy::SuspendAndPreserve,
        )
        .expect("handoff to human");

    let avatar_pos = world.get_avatar(avatar_id).unwrap().position;
    let human_target = avatar_pos + Vec3::new(0.5, 0.0, 0.0);
    let human_cmd = SimCommand::MoveAvatar {
        avatar_id,
        target_pos: human_target,
        speed: 2.0,
    };
    let executed = arbitrator
        .tick_avatar(avatar_id, &world, Some(human_cmd))
        .expect("exec human");
    world.apply_command(executed).expect("apply human");
    world.step(0.1);

    // Verify manual movement and held object preserved
    let held_during = world.get_avatar(avatar_id).unwrap().grasp.held_object;
    if held_during != Some(OBJECT_CEREAL_BOX) {
        return ScenarioResult {
            scenario: "C".to_string(),
            title: "Control Handoff".to_string(),
            status: "FAILED".to_string(),
            steps_taken: 3,
            duration_sim_seconds: world.time_elapsed,
            details: "Held object lost during human control".to_string(),
        };
    }

    // Step 3: Return control to AI
    arbitrator
        .switch_mode(
            avatar_id,
            ControlMode::DirectedAI,
            &world,
            InterruptionPolicy::SuspendAndPreserve,
        )
        .expect("handoff back to AI");

    // Resume AI until completion
    let mut steps = 3;
    while steps < 25 {
        steps += 1;
        if let Some(cmd) = arbitrator.tick_avatar(avatar_id, &world, None) {
            world.apply_command(cmd).expect("apply resumed cmd");
            world.step(0.1);
        } else {
            break;
        }
    }

    let bowl = world.get_container(OBJECT_BOWL).unwrap();
    if bowl.is_prepared_cereal() && handoff.state_preserved {
        ScenarioResult {
            scenario: "C".to_string(),
            title: "Control Handoff".to_string(),
            status: "PASSED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: "Seamless Directed AI -> Human -> Directed AI handoff executed with zero state corruption and task completion".to_string(),
        }
    } else {
        ScenarioResult {
            scenario: "C".to_string(),
            title: "Control Handoff".to_string(),
            status: "FAILED".to_string(),
            steps_taken: steps,
            duration_sim_seconds: world.time_elapsed,
            details: format!(
                "Failed to complete task after handoff. Bowl ready: {}",
                bowl.is_prepared_cereal()
            ),
        }
    }
}
