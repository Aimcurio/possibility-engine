use crate::containers::Container;
use crate::gravity::GravityDevice;
use crate::manipulation::GraspSocket;
use crate::physics::RigidBody;
use possibility_core::{
    AvatarId, CommandValidationError, InvariantViolation, ObjectId, SimCommand, SimEvent, Vec3,
    WorldBounds,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const OBJECT_BOWL: ObjectId = ObjectId(1);
pub const OBJECT_CEREAL_BOX: ObjectId = ObjectId(2);
pub const OBJECT_MILK_CARTON: ObjectId = ObjectId(3);
pub const OBJECT_GRAVITY_DEVICE: ObjectId = ObjectId(10);
pub const OBJECT_TEST_CUBE_A: ObjectId = ObjectId(20);
pub const OBJECT_TEST_CUBE_B: ObjectId = ObjectId(21);
pub const OBJECT_TEST_SPHERE: ObjectId = ObjectId(22);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvatarSimRecord {
    pub id: AvatarId,
    pub name: String,
    pub position: Vec3,
    pub grasp: GraspSocket,
    pub speed: f32,
    pub initial_position: Vec3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectSimRecord {
    pub id: ObjectId,
    pub name: String,
    pub body: RigidBody,
    pub is_grabbable: bool,
    pub is_device: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimWorld {
    pub avatars: HashMap<AvatarId, AvatarSimRecord>,
    pub objects: HashMap<ObjectId, ObjectSimRecord>,
    pub containers: HashMap<ObjectId, Container>,
    pub gravity_devices: HashMap<ObjectId, GravityDevice>,
    pub bounds: WorldBounds,
    pub tick_counter: u64,
    pub time_elapsed: f32,
    pub initial_total_cereal: f32,
    pub initial_total_milk: f32,
}

impl SimWorld {
    pub fn new_laboratory() -> Self {
        let mut avatars = HashMap::new();
        // Three selectable avatars
        avatars.insert(
            AvatarId(0),
            AvatarSimRecord {
                id: AvatarId(0),
                name: "Alpha".to_string(),
                position: Vec3::new(-3.0, 0.0, 0.0),
                grasp: GraspSocket::default(),
                speed: 3.5,
                initial_position: Vec3::new(-3.0, 0.0, 0.0),
            },
        );
        avatars.insert(
            AvatarId(1),
            AvatarSimRecord {
                id: AvatarId(1),
                name: "Beta".to_string(),
                position: Vec3::new(0.0, 0.0, 0.0),
                grasp: GraspSocket::default(),
                speed: 3.0,
                initial_position: Vec3::new(0.0, 0.0, 0.0),
            },
        );
        avatars.insert(
            AvatarId(2),
            AvatarSimRecord {
                id: AvatarId(2),
                name: "Gamma".to_string(),
                position: Vec3::new(3.0, 0.0, 0.0),
                grasp: GraspSocket::default(),
                speed: 4.0,
                initial_position: Vec3::new(3.0, 0.0, 0.0),
            },
        );

        let mut objects = HashMap::new();
        let mut containers = HashMap::new();
        let mut gravity_devices = HashMap::new();

        // Scenario A Objects: Lab table counter at X: -2.0 to 0.0, Z: 3.0
        // Bowl on table
        objects.insert(
            OBJECT_BOWL,
            ObjectSimRecord {
                id: OBJECT_BOWL,
                name: "Ceramic Bowl".to_string(),
                body: RigidBody::new(Vec3::new(-1.0, 0.85, 3.0), 0.5, false),
                is_grabbable: true,
                is_device: false,
            },
        );
        containers.insert(OBJECT_BOWL, Container::new_bowl());

        // Cereal box on table
        objects.insert(
            OBJECT_CEREAL_BOX,
            ObjectSimRecord {
                id: OBJECT_CEREAL_BOX,
                name: "Crunchy Flakes".to_string(),
                body: RigidBody::new(Vec3::new(-1.8, 0.85, 3.0), 0.6, false),
                is_grabbable: true,
                is_device: false,
            },
        );
        containers.insert(OBJECT_CEREAL_BOX, Container::new_cereal_box());

        // Milk carton on table
        objects.insert(
            OBJECT_MILK_CARTON,
            ObjectSimRecord {
                id: OBJECT_MILK_CARTON,
                name: "Fresh Milk Carton".to_string(),
                body: RigidBody::new(Vec3::new(-0.2, 0.85, 3.0), 1.0, false),
                is_grabbable: true,
                is_device: false,
            },
        );
        containers.insert(OBJECT_MILK_CARTON, Container::new_milk_carton());

        // Scenario B Objects: Experimental Gravity Field in containment zone at (5.0, 1.0, 2.0)
        let gravity_center = Vec3::new(6.0, 1.0, 2.0);
        objects.insert(
            OBJECT_GRAVITY_DEVICE,
            ObjectSimRecord {
                id: OBJECT_GRAVITY_DEVICE,
                name: "Graviton Core Generator".to_string(),
                body: RigidBody::new(gravity_center, 100.0, true),
                is_grabbable: false,
                is_device: true,
            },
        );
        gravity_devices.insert(
            OBJECT_GRAVITY_DEVICE,
            GravityDevice::new(OBJECT_GRAVITY_DEVICE, gravity_center, 7.0, 35.0),
        );

        // Movable test objects in the gravity containment chamber
        objects.insert(
            OBJECT_TEST_CUBE_A,
            ObjectSimRecord {
                id: OBJECT_TEST_CUBE_A,
                name: "Magnetic Alloy Cube Alpha".to_string(),
                body: RigidBody::new(Vec3::new(4.5, 0.5, 2.0), 1.5, false),
                is_grabbable: true,
                is_device: false,
            },
        );
        objects.insert(
            OBJECT_TEST_CUBE_B,
            ObjectSimRecord {
                id: OBJECT_TEST_CUBE_B,
                name: "Magnetic Alloy Cube Beta".to_string(),
                body: RigidBody::new(Vec3::new(7.5, 0.5, 3.0), 1.5, false),
                is_grabbable: true,
                is_device: false,
            },
        );
        objects.insert(
            OBJECT_TEST_SPHERE,
            ObjectSimRecord {
                id: OBJECT_TEST_SPHERE,
                name: "Resonance Test Sphere".to_string(),
                body: RigidBody::new(Vec3::new(6.0, 0.5, 4.2), 2.0, false),
                is_grabbable: true,
                is_device: false,
            },
        );

        Self {
            avatars,
            objects,
            containers,
            gravity_devices,
            bounds: WorldBounds::default(),
            tick_counter: 0,
            time_elapsed: 0.0,
            initial_total_cereal: 500.0,
            initial_total_milk: 1000.0,
        }
    }

    pub fn get_avatar(&self, id: AvatarId) -> Option<&AvatarSimRecord> {
        self.avatars.get(&id)
    }

    pub fn get_object(&self, id: ObjectId) -> Option<&ObjectSimRecord> {
        self.objects.get(&id)
    }

    pub fn get_container(&self, id: ObjectId) -> Option<&Container> {
        self.containers.get(&id)
    }

    pub fn get_gravity_device(&self, id: ObjectId) -> Option<&GravityDevice> {
        self.gravity_devices.get(&id)
    }

    pub fn step(&mut self, dt: f32) -> Vec<SimEvent> {
        let mut events = Vec::new();
        self.tick_counter += 1;
        self.time_elapsed += dt;

        // Collect active gravity forces
        let mut external_accels: HashMap<ObjectId, Vec3> = HashMap::new();
        for dev in self.gravity_devices.values() {
            if dev.is_active {
                for (obj_id, obj) in &self.objects {
                    if !obj.body.is_static {
                        let accel = dev.calculate_field_acceleration(obj.body.position);
                        let entry = external_accels.entry(*obj_id).or_insert(Vec3::ZERO);
                        *entry = *entry + accel;
                    }
                }
            }
        }

        // Identify which objects are currently grasped
        let mut held_objects: HashMap<ObjectId, AvatarId> = HashMap::new();
        for avatar in self.avatars.values() {
            if let Some(held_id) = avatar.grasp.held_object {
                held_objects.insert(held_id, avatar.id);
            }
        }

        // Step physics for all objects
        for (obj_id, obj) in self.objects.iter_mut() {
            if let Some(avatar_id) = held_objects.get(obj_id) {
                // Held object moves with avatar socket
                if let Some(av) = self.avatars.get(avatar_id) {
                    obj.body.position = av.grasp.calculate_held_world_pos(av.position);
                    obj.body.velocity = Vec3::ZERO;
                }
            } else {
                // Check velocity invariant before step
                if let Err(violation) = self.bounds.validate_velocity(*obj_id, obj.body.velocity) {
                    events.push(SimEvent::InvariantViolationDetected(violation));
                }

                let external_accel = external_accels.get(obj_id).copied().unwrap_or(Vec3::ZERO);
                if let Some(excessive_spd) = obj.body.step(external_accel, dt, &self.bounds) {
                    events.push(SimEvent::InvariantViolationDetected(
                        InvariantViolation::ExcessiveVelocity {
                            object_id: *obj_id,
                            velocity: obj.body.velocity.normalize() * excessive_spd,
                            max_speed: self.bounds.max_velocity,
                        },
                    ));
                }
            }

            // Invariant checks on position
            if let Err(violation) = self.bounds.validate_position(*obj_id, obj.body.position) {
                events.push(SimEvent::InvariantViolationDetected(violation));
            }
        }

        // Check container invariants
        let mut current_total_cereal = 0.0;
        let mut current_total_milk = 0.0;
        for (c_id, container) in &self.containers {
            if container.cereal_grams < 0.0 {
                events.push(SimEvent::InvariantViolationDetected(
                    InvariantViolation::NegativeVolume {
                        object_id: *c_id,
                        volume: container.cereal_grams,
                    },
                ));
            }
            if container.milk_ml < 0.0 {
                events.push(SimEvent::InvariantViolationDetected(
                    InvariantViolation::NegativeVolume {
                        object_id: *c_id,
                        volume: container.milk_ml,
                    },
                ));
            }
            current_total_cereal += container.cereal_grams;
            current_total_milk += container.milk_ml;
        }

        // Conservation check
        let cereal_delta = (current_total_cereal - self.initial_total_cereal).abs();
        if cereal_delta > 1e-3 {
            events.push(SimEvent::InvariantViolationDetected(
                InvariantViolation::MassConservationViolation {
                    expected_total: self.initial_total_cereal,
                    actual_total: current_total_cereal,
                    delta: cereal_delta,
                },
            ));
        }

        let milk_delta = (current_total_milk - self.initial_total_milk).abs();
        if milk_delta > 1e-3 {
            events.push(SimEvent::InvariantViolationDetected(
                InvariantViolation::MassConservationViolation {
                    expected_total: self.initial_total_milk,
                    actual_total: current_total_milk,
                    delta: milk_delta,
                },
            ));
        }

        events
    }

    pub fn apply_command(
        &mut self,
        cmd: SimCommand,
    ) -> Result<Vec<SimEvent>, CommandValidationError> {
        let mut events = Vec::new();

        match cmd {
            SimCommand::MoveAvatar {
                avatar_id,
                target_pos,
                speed,
            } => {
                let avatar = self
                    .avatars
                    .get_mut(&avatar_id)
                    .ok_or(CommandValidationError::AvatarNotFound(avatar_id))?;
                avatar.position = target_pos;
                avatar.speed = speed;
                events.push(SimEvent::AvatarMoved {
                    avatar_id,
                    position: target_pos,
                });
            }

            SimCommand::PickUpObject {
                avatar_id,
                object_id,
            } => {
                let avatar = self
                    .avatars
                    .get(&avatar_id)
                    .ok_or(CommandValidationError::AvatarNotFound(avatar_id))?;
                let obj = self
                    .objects
                    .get(&object_id)
                    .ok_or(CommandValidationError::ObjectNotFound(object_id))?;

                if !obj.is_grabbable {
                    return Err(CommandValidationError::DeviceNotRecognized(object_id));
                }

                let dist = avatar.position.distance(obj.body.position);
                if dist > 3.0 {
                    return Err(CommandValidationError::TargetOutOfReach {
                        distance: dist,
                        max_reach: 3.0,
                    });
                }

                // Check if already held by any other avatar
                for (other_id, other_avatar) in &self.avatars {
                    if other_avatar.grasp.held_object == Some(object_id) {
                        return Err(CommandValidationError::ObjectAlreadyHeld {
                            object_id,
                            held_by: *other_id,
                        });
                    }
                }

                let avatar_mut = self.avatars.get_mut(&avatar_id).unwrap();
                if avatar_mut.grasp.attach(object_id).is_err() {
                    return Err(CommandValidationError::ObjectAlreadyHeld {
                        object_id,
                        held_by: avatar_id,
                    });
                }

                events.push(SimEvent::ObjectPickedUp {
                    avatar_id,
                    object_id,
                });
            }

            SimCommand::PlaceObject {
                avatar_id,
                object_id,
                position,
            } => {
                let avatar = self
                    .avatars
                    .get_mut(&avatar_id)
                    .ok_or(CommandValidationError::AvatarNotFound(avatar_id))?;

                if avatar.grasp.held_object != Some(object_id) {
                    return Err(CommandValidationError::ObjectNotHeld {
                        avatar_id,
                        object_id,
                    });
                }

                avatar.grasp.detach();
                if let Some(obj) = self.objects.get_mut(&object_id) {
                    obj.body.position = position;
                    obj.body.velocity = Vec3::ZERO;
                }

                events.push(SimEvent::ObjectPlaced {
                    avatar_id,
                    object_id,
                    position,
                });
            }

            SimCommand::PourLiquid {
                avatar_id,
                source,
                target,
                amount_ml,
            } => {
                if !self.avatars.contains_key(&avatar_id) {
                    return Err(CommandValidationError::AvatarNotFound(avatar_id));
                }

                let (_src_cereal, src_milk) = {
                    let src = self
                        .containers
                        .get(&source)
                        .ok_or(CommandValidationError::ObjectNotFound(source))?;
                    (src.cereal_grams, src.milk_ml)
                };

                if src_milk < amount_ml && src_milk <= 0.0 {
                    return Err(CommandValidationError::InsufficientContents {
                        object_id: source,
                        requested: amount_ml,
                        available: src_milk,
                    });
                }

                let (transferred_cereal, transferred_milk) = {
                    // Extract mutable references separately
                    let mut src_clone = self.containers.get(&source).cloned().unwrap();
                    let mut tgt_clone = self
                        .containers
                        .get(&target)
                        .cloned()
                        .ok_or(CommandValidationError::ObjectNotFound(target))?;

                    let poured = src_clone
                        .pour_milk(&mut tgt_clone, amount_ml)
                        .map_err(|_| CommandValidationError::ContainerFull {
                            object_id: target,
                            capacity: tgt_clone.capacity_ml,
                            current: tgt_clone.current_liquid_and_cereal_volume(),
                        })?;

                    self.containers.insert(source, src_clone);
                    self.containers.insert(target, tgt_clone);
                    (0.0, poured)
                };

                events.push(SimEvent::ContentsTransferred {
                    source,
                    target,
                    cereal_amount_g: transferred_cereal,
                    milk_amount_ml: transferred_milk,
                });
            }

            SimCommand::DispenseCereal {
                avatar_id,
                source,
                target,
                amount_g,
            } => {
                if !self.avatars.contains_key(&avatar_id) {
                    return Err(CommandValidationError::AvatarNotFound(avatar_id));
                }

                let (transferred_cereal, transferred_milk) = {
                    let mut src_clone = self
                        .containers
                        .get(&source)
                        .cloned()
                        .ok_or(CommandValidationError::ObjectNotFound(source))?;
                    let mut tgt_clone = self
                        .containers
                        .get(&target)
                        .cloned()
                        .ok_or(CommandValidationError::ObjectNotFound(target))?;

                    let dispensed = src_clone
                        .dispense_cereal(&mut tgt_clone, amount_g)
                        .map_err(|_| CommandValidationError::ContainerFull {
                            object_id: target,
                            capacity: tgt_clone.capacity_ml,
                            current: tgt_clone.current_liquid_and_cereal_volume(),
                        })?;

                    self.containers.insert(source, src_clone);
                    self.containers.insert(target, tgt_clone);
                    (dispensed, 0.0)
                };

                events.push(SimEvent::ContentsTransferred {
                    source,
                    target,
                    cereal_amount_g: transferred_cereal,
                    milk_amount_ml: transferred_milk,
                });
            }

            SimCommand::SetGravityDeviceState {
                avatar_id,
                device_id,
                active,
            } => {
                let avatar = self
                    .avatars
                    .get(&avatar_id)
                    .ok_or(CommandValidationError::AvatarNotFound(avatar_id))?;
                let dev = self
                    .gravity_devices
                    .get_mut(&device_id)
                    .ok_or(CommandValidationError::DeviceNotRecognized(device_id))?;

                let dist = avatar.position.distance(dev.center_pos);
                if dist > 6.0 {
                    return Err(CommandValidationError::TargetOutOfReach {
                        distance: dist,
                        max_reach: 6.0,
                    });
                }

                dev.set_active(active);
                let radius = dev.field_radius;
                events.push(SimEvent::GravityDeviceStateChanged {
                    device_id,
                    active,
                    field_radius: radius,
                });
            }

            SimCommand::ResetSimulation => {
                self.reset();
                events.push(SimEvent::SimulationReset);
            }
        }

        Ok(events)
    }

    pub fn reset(&mut self) {
        *self = Self::new_laboratory();
    }
}
