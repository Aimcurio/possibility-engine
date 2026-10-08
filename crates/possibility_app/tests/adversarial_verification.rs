use possibility_cognition::{
    AvatarAttributes, AvatarGoal, DeterministicPlanner, NaturalLanguageInterpreter,
    PerceptionSnapshot, TaskStep,
};
use possibility_core::{AvatarId, CommandValidationError, ObjectId, SimCommand, Vec3};
use possibility_sim::{
    SimWorld, OBJECT_BOWL, OBJECT_CEREAL_BOX, OBJECT_GRAVITY_DEVICE, OBJECT_TEST_CUBE_A,
};

#[test]
fn test_adversarial_invalid_commands_and_missing_objects() {
    let mut world = SimWorld::new_laboratory();
    let avatar_id = AvatarId(0);

    // 1. Missing avatar
    let missing_av_cmd = SimCommand::MoveAvatar {
        avatar_id: AvatarId(999),
        target_pos: Vec3::ZERO,
        speed: 1.0,
    };
    assert_eq!(
        world.apply_command(missing_av_cmd),
        Err(CommandValidationError::AvatarNotFound(AvatarId(999)))
    );

    // 2. Missing object pickup
    let missing_obj_cmd = SimCommand::PickUpObject {
        avatar_id,
        object_id: ObjectId(9999),
    };
    assert_eq!(
        world.apply_command(missing_obj_cmd),
        Err(CommandValidationError::ObjectNotFound(ObjectId(9999)))
    );

    // 3. Unreachable object pickup (Avatar is at (-3, 0, 0), bowl is at (-1, 0.85, 3))
    let unreachable_cmd = SimCommand::PickUpObject {
        avatar_id,
        object_id: OBJECT_BOWL,
    };
    match world.apply_command(unreachable_cmd) {
        Err(CommandValidationError::TargetOutOfReach {
            distance,
            max_reach,
        }) => {
            assert!(distance > max_reach);
        }
        other => panic!("Expected TargetOutOfReach, got {:?}", other),
    }

    // 4. Ungrabbable device pickup
    let table_pos = Vec3::new(6.0, 0.0, 2.0); // directly next to gravity generator
    world
        .apply_command(SimCommand::MoveAvatar {
            avatar_id,
            target_pos: table_pos,
            speed: 3.0,
        })
        .unwrap();

    let grab_device_cmd = SimCommand::PickUpObject {
        avatar_id,
        object_id: OBJECT_GRAVITY_DEVICE,
    };
    assert_eq!(
        world.apply_command(grab_device_cmd),
        Err(CommandValidationError::DeviceNotRecognized(
            OBJECT_GRAVITY_DEVICE
        ))
    );

    // 5. Placing an object that is not held
    let place_not_held_cmd = SimCommand::PlaceObject {
        avatar_id,
        object_id: OBJECT_CEREAL_BOX,
        position: Vec3::ZERO,
    };
    assert_eq!(
        world.apply_command(place_not_held_cmd),
        Err(CommandValidationError::ObjectNotHeld {
            avatar_id,
            object_id: OBJECT_CEREAL_BOX
        })
    );
}

#[test]
fn test_adversarial_concurrent_avatar_interactions() {
    let mut world = SimWorld::new_laboratory();
    let av0 = AvatarId(0);
    let av1 = AvatarId(1);

    // Move both avatars near the cereal box
    let table_pos = Vec3::new(-1.8, 0.0, 2.0);
    world
        .apply_command(SimCommand::MoveAvatar {
            avatar_id: av0,
            target_pos: table_pos,
            speed: 3.0,
        })
        .unwrap();
    world
        .apply_command(SimCommand::MoveAvatar {
            avatar_id: av1,
            target_pos: table_pos,
            speed: 3.0,
        })
        .unwrap();

    // Avatar 0 picks up the cereal box
    world
        .apply_command(SimCommand::PickUpObject {
            avatar_id: av0,
            object_id: OBJECT_CEREAL_BOX,
        })
        .expect("av0 picks up cereal box");

    // Avatar 1 attempts to steal/pick up the SAME cereal box while av0 holds it
    let steal_attempt = SimCommand::PickUpObject {
        avatar_id: av1,
        object_id: OBJECT_CEREAL_BOX,
    };
    assert_eq!(
        world.apply_command(steal_attempt),
        Err(CommandValidationError::ObjectAlreadyHeld {
            object_id: OBJECT_CEREAL_BOX,
            held_by: av0,
        })
    );
}

#[test]
fn test_adversarial_repeated_task_execution_and_reset() {
    let mut world = SimWorld::new_laboratory();
    let planner = DeterministicPlanner::new();
    let avatar_id = AvatarId(0);
    let goal = AvatarGoal::PrepareBowlOfCereal;

    // Run cycle 1
    let mut steps_1 = 0;
    while steps_1 < 25 {
        steps_1 += 1;
        let snap = PerceptionSnapshot::capture(&world, avatar_id).unwrap();
        let step = planner.plan_next_step(&goal, &snap);
        if matches!(step, TaskStep::Complete) {
            break;
        }
        if let Some(cmd) = planner.step_to_command(avatar_id, &step, 3.5) {
            world.apply_command(cmd).unwrap();
        }
        world.step(0.1);
    }
    assert!(world
        .get_container(OBJECT_BOWL)
        .unwrap()
        .is_prepared_cereal());

    // Reset world
    world.apply_command(SimCommand::ResetSimulation).unwrap();
    assert!(!world
        .get_container(OBJECT_BOWL)
        .unwrap()
        .is_prepared_cereal());
    assert_eq!(world.get_container(OBJECT_BOWL).unwrap().cereal_grams, 0.0);

    // Run cycle 2
    let mut steps_2 = 0;
    while steps_2 < 25 {
        steps_2 += 1;
        let snap = PerceptionSnapshot::capture(&world, avatar_id).unwrap();
        let step = planner.plan_next_step(&goal, &snap);
        if matches!(step, TaskStep::Complete) {
            break;
        }
        if let Some(cmd) = planner.step_to_command(avatar_id, &step, 3.5) {
            world.apply_command(cmd).unwrap();
        }
        world.step(0.1);
    }
    assert!(world
        .get_container(OBJECT_BOWL)
        .unwrap()
        .is_prepared_cereal());
    assert_eq!(
        steps_1, steps_2,
        "Repeated execution must take identical deterministic steps"
    );
}

#[test]
fn test_adversarial_nlp_and_attribute_robustness() {
    // Malformed and edge-case NLP instructions
    assert!(NaturalLanguageInterpreter::interpret_instruction("").is_err());
    assert!(NaturalLanguageInterpreter::interpret_instruction("   ").is_err());
    assert!(
        NaturalLanguageInterpreter::interpret_instruction("gibberish blabla xyzt 12345").is_err()
    );

    let mut attrs = AvatarAttributes::default();
    let initial_speed = attrs.speed_multiplier;
    let changed = NaturalLanguageInterpreter::interpret_attributes(
        "random noise without keywords",
        &mut attrs,
    );
    assert!(!changed);
    assert_eq!(attrs.speed_multiplier, initial_speed);
}

#[test]
fn test_adversarial_bounds_and_velocity_clamping() {
    let mut world = SimWorld::new_laboratory();

    // Create an object with extreme velocity exceeding max allowed
    let obj = world.objects.get_mut(&OBJECT_TEST_CUBE_A).unwrap();
    obj.body.velocity = Vec3::new(500.0, 500.0, 500.0); // well above 50.0 m/s limit

    let events = world.step(0.1);
    let violation_found = events.iter().any(|e| {
        matches!(
            e,
            possibility_core::SimEvent::InvariantViolationDetected(
                possibility_core::InvariantViolation::ExcessiveVelocity { .. }
            )
        )
    });
    assert!(
        violation_found,
        "System must detect and flag excessive velocity invariant violation"
    );

    // Check that velocity was clamped within bounds
    let clamped_speed = world
        .get_object(OBJECT_TEST_CUBE_A)
        .unwrap()
        .body
        .velocity
        .length();
    assert!(clamped_speed <= world.bounds.max_velocity + 1e-4);
}
