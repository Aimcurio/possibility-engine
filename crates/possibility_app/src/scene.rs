use bevy::prelude::*;
use possibility_core::{AvatarId, ObjectId};
use possibility_sim::{
    OBJECT_BOWL, OBJECT_CEREAL_BOX, OBJECT_GRAVITY_DEVICE, OBJECT_MILK_CARTON, OBJECT_TEST_CUBE_A,
    OBJECT_TEST_CUBE_B, OBJECT_TEST_SPHERE,
};

#[derive(Component)]
pub struct AvatarVisual {
    pub id: AvatarId,
}

#[derive(Component)]
pub struct ObjectVisual {
    pub id: ObjectId,
}

#[derive(Component)]
pub struct GravityCoreVisual;

pub fn setup_3d_laboratory(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.5, 12.0).looking_at(Vec3::new(1.0, 1.0, 1.5), Vec3::Y),
    ));

    // 2. Lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.9, 1.0),
        brightness: 350.0,
    });

    // 3. Laboratory Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.20, 0.24),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 4. Lab Workbench / Table (Scenario A prep station)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.5, 0.8, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.38, 0.42),
            metallic: 0.4,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(-1.0, 0.4, 3.0),
    ));

    // 5. Scenario A: Ceramic Bowl
    commands.spawn((
        ObjectVisual { id: OBJECT_BOWL },
        Mesh3d(meshes.add(Cylinder::new(0.35, 0.25))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.95, 0.98),
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(-1.0, 0.85, 3.0),
    ));

    // 6. Scenario A: Cereal Box (Warm orange/gold)
    commands.spawn((
        ObjectVisual {
            id: OBJECT_CEREAL_BOX,
        },
        Mesh3d(meshes.add(Cuboid::new(0.25, 0.45, 0.15))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.55, 0.10),
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(-1.8, 0.85, 3.0),
    ));

    // 7. Scenario A: Milk Carton (Crisp white / light blue)
    commands.spawn((
        ObjectVisual {
            id: OBJECT_MILK_CARTON,
        },
        Mesh3d(meshes.add(Cuboid::new(0.22, 0.40, 0.22))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.92, 1.0),
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_xyz(-0.2, 0.85, 3.0),
    ));

    // 8. Scenario B: Artificial Gravity Generator Core
    // Base pedestal
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(1.2, 0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.18, 0.22),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(6.0, 0.3, 2.0),
    ));

    // Glowing energy core
    commands.spawn((
        ObjectVisual {
            id: OBJECT_GRAVITY_DEVICE,
        },
        GravityCoreVisual,
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.8, 1.0),
            emissive: LinearRgba::new(0.2, 1.5, 2.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(6.0, 1.0, 2.0),
    ));

    // 9. Scenario B: Test Objects in containment zone
    commands.spawn((
        ObjectVisual {
            id: OBJECT_TEST_CUBE_A,
        },
        Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            metallic: 0.6,
            ..default()
        })),
        Transform::from_xyz(4.5, 0.5, 2.0),
    ));

    commands.spawn((
        ObjectVisual {
            id: OBJECT_TEST_CUBE_B,
        },
        Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.3),
            metallic: 0.6,
            ..default()
        })),
        Transform::from_xyz(7.5, 0.5, 3.0),
    ));

    commands.spawn((
        ObjectVisual {
            id: OBJECT_TEST_SPHERE,
        },
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.8, 0.1),
            metallic: 0.9,
            ..default()
        })),
        Transform::from_xyz(6.0, 0.5, 4.2),
    ));

    // 10. Three Selectable Avatars
    // Avatar 0: Alpha (Scientist - Deep Sapphire Blue)
    commands.spawn((
        AvatarVisual { id: AvatarId(0) },
        Mesh3d(meshes.add(Capsule3d::new(0.35, 1.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.45, 0.95),
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(-3.0, 0.9, 0.0),
    ));

    // Avatar 1: Beta (Engineer - Radiant Amber Gold)
    commands.spawn((
        AvatarVisual { id: AvatarId(1) },
        Mesh3d(meshes.add(Capsule3d::new(0.35, 1.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.75, 0.15),
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.9, 0.0),
    ));

    // Avatar 2: Gamma (Explorer - Emerald Green)
    commands.spawn((
        AvatarVisual { id: AvatarId(2) },
        Mesh3d(meshes.add(Capsule3d::new(0.35, 1.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.85, 0.45),
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(3.0, 0.9, 0.0),
    ));
}
