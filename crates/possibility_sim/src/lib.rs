pub mod containers;
pub mod gravity;
pub mod manipulation;
pub mod physics;
pub mod world;

pub use containers::Container;
pub use gravity::GravityDevice;
pub use manipulation::{AssemblyComponent, GraspSocket};
pub use physics::RigidBody;
pub use world::{
    AvatarSimRecord, ObjectSimRecord, SimWorld, OBJECT_BOWL, OBJECT_CEREAL_BOX,
    OBJECT_GRAVITY_DEVICE, OBJECT_MILK_CARTON, OBJECT_TEST_CUBE_A, OBJECT_TEST_CUBE_B,
    OBJECT_TEST_SPHERE,
};

#[cfg(test)]
mod tests {
    use super::*;
    use possibility_core::{AvatarId, SimCommand, Vec3};

    #[test]
    fn test_scenario_a_authoritative_simulation() {
        let mut world = SimWorld::new_laboratory();
        let avatar_id = AvatarId(0);

        // Avatar navigates to counter table near bowl and cereal
        let table_pos = Vec3::new(-1.0, 0.0, 2.0);
        world
            .apply_command(SimCommand::MoveAvatar {
                avatar_id,
                target_pos: table_pos,
                speed: 3.0,
            })
            .expect("move avatar");

        // Pick up cereal box
        world
            .apply_command(SimCommand::PickUpObject {
                avatar_id,
                object_id: OBJECT_CEREAL_BOX,
            })
            .expect("pickup cereal box");

        // Dispense 45g cereal into bowl
        world
            .apply_command(SimCommand::DispenseCereal {
                avatar_id,
                source: OBJECT_CEREAL_BOX,
                target: OBJECT_BOWL,
                amount_g: 45.0,
            })
            .expect("dispense cereal");

        // Place down cereal box
        world
            .apply_command(SimCommand::PlaceObject {
                avatar_id,
                object_id: OBJECT_CEREAL_BOX,
                position: Vec3::new(-1.8, 0.85, 3.0),
            })
            .expect("place cereal box");

        // Pick up milk carton
        world
            .apply_command(SimCommand::PickUpObject {
                avatar_id,
                object_id: OBJECT_MILK_CARTON,
            })
            .expect("pickup milk carton");

        // Pour 120ml milk into bowl
        world
            .apply_command(SimCommand::PourLiquid {
                avatar_id,
                source: OBJECT_MILK_CARTON,
                target: OBJECT_BOWL,
                amount_ml: 120.0,
            })
            .expect("pour milk");

        // Place down milk carton
        world
            .apply_command(SimCommand::PlaceObject {
                avatar_id,
                object_id: OBJECT_MILK_CARTON,
                position: Vec3::new(-0.2, 0.85, 3.0),
            })
            .expect("place milk carton");

        // Authoritative verification of end state
        let bowl = world.get_container(OBJECT_BOWL).expect("bowl exists");
        assert_eq!(bowl.cereal_grams, 45.0);
        assert_eq!(bowl.milk_ml, 120.0);
        assert!(bowl.is_prepared_cereal());

        // Invariant check: mass conservation
        let events = world.step(1.0 / 60.0);
        let violations: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, possibility_core::SimEvent::InvariantViolationDetected(_)))
            .collect();
        assert!(
            violations.is_empty(),
            "No invariant violations during Scenario A"
        );
    }

    #[test]
    fn test_scenario_b_gravity_lifecycle() {
        let mut world = SimWorld::new_laboratory();
        let avatar_id = AvatarId(1);

        // Move avatar to gravity device terminal
        world
            .apply_command(SimCommand::MoveAvatar {
                avatar_id,
                target_pos: Vec3::new(4.5, 0.0, 1.5),
                speed: 3.0,
            })
            .expect("move avatar");

        // Check initial position of test cube
        let initial_cube_pos = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
        assert!(
            !world
                .get_gravity_device(OBJECT_GRAVITY_DEVICE)
                .unwrap()
                .is_active
        );

        // Step simulation without gravity active: cube remains at initial position
        world.step(0.1);
        let cube_pos_idle = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
        assert_eq!(initial_cube_pos, cube_pos_idle);

        // Activate gravity device
        world
            .apply_command(SimCommand::SetGravityDeviceState {
                avatar_id,
                device_id: OBJECT_GRAVITY_DEVICE,
                active: true,
            })
            .expect("activate device");

        assert!(
            world
                .get_gravity_device(OBJECT_GRAVITY_DEVICE)
                .unwrap()
                .is_active
        );

        // Step simulation: cube accelerates towards device center (6.0, 1.0, 2.0)
        for _ in 0..10 {
            world.step(0.05);
        }

        let cube_pos_active = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
        assert!(
            cube_pos_active.x > initial_cube_pos.x,
            "Cube should move along +X towards device center at x=6.0"
        );

        // Deactivate gravity device
        world
            .apply_command(SimCommand::SetGravityDeviceState {
                avatar_id,
                device_id: OBJECT_GRAVITY_DEVICE,
                active: false,
            })
            .expect("deactivate device");

        assert!(
            !world
                .get_gravity_device(OBJECT_GRAVITY_DEVICE)
                .unwrap()
                .is_active
        );

        // Reset simulation: restores all objects to initial pristine positions
        world
            .apply_command(SimCommand::ResetSimulation)
            .expect("reset simulation");
        let reset_cube_pos = world.get_object(OBJECT_TEST_CUBE_A).unwrap().body.position;
        assert_eq!(reset_cube_pos, initial_cube_pos);
    }
}
