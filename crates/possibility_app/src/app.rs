use bevy::prelude::*;
use possibility_control::ControlArbitrator;
use possibility_core::AvatarId;
use possibility_sim::{SimWorld, OBJECT_GRAVITY_DEVICE};

use crate::player_input::{handle_player_input, SelectedAvatar};
use crate::scene::{setup_3d_laboratory, AvatarVisual, GravityCoreVisual, ObjectVisual};
use crate::ui::{setup_hud, update_hud};

#[derive(Resource)]
pub struct SimWorldResource(pub SimWorld);

#[derive(Resource)]
pub struct ArbitratorResource(pub ControlArbitrator);

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "THE POSSIBILITY ENGINE — 3D Interactive Sandbox".to_string(),
            resolution: (1280.0_f32, 720.0_f32).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(SimWorldResource(SimWorld::new_laboratory()))
    .insert_resource(ArbitratorResource(ControlArbitrator::new()))
    .init_resource::<SelectedAvatar>()
    .add_systems(Startup, (setup_3d_laboratory, setup_hud))
    .add_systems(
        Update,
        (
            handle_player_input,
            tick_simulation_bridge,
            sync_visual_transforms,
            update_hud,
        )
            .chain(),
    );

    app
}

fn tick_simulation_bridge(
    time: Res<Time>,
    mut world_res: ResMut<SimWorldResource>,
    mut arb_res: ResMut<ArbitratorResource>,
) {
    let dt = time.delta_secs().min(0.05);

    // Tick AI avatars
    for avatar_id in [AvatarId(0), AvatarId(1), AvatarId(2)] {
        if let Some(cmd) = arb_res.0.tick_avatar(avatar_id, &world_res.0, None) {
            let _ = world_res.0.apply_command(cmd);
        }
    }

    // Step physics and containers
    world_res.0.step(dt);
}

fn sync_visual_transforms(
    world_res: Res<SimWorldResource>,
    mut avatar_query: Query<(&AvatarVisual, &mut Transform), Without<ObjectVisual>>,
    mut object_query: Query<(&ObjectVisual, &mut Transform), Without<AvatarVisual>>,
    gravity_core_query: Query<&MeshMaterial3d<StandardMaterial>, With<GravityCoreVisual>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Sync avatars
    for (vis, mut transform) in avatar_query.iter_mut() {
        if let Some(av) = world_res.0.get_avatar(vis.id) {
            transform.translation = Vec3::new(av.position.x, av.position.y + 0.9, av.position.z);
        }
    }

    // 2. Sync objects
    for (vis, mut transform) in object_query.iter_mut() {
        if let Some(obj) = world_res.0.get_object(vis.id) {
            transform.translation = Vec3::new(
                obj.body.position.x,
                obj.body.position.y,
                obj.body.position.z,
            );
        }
    }

    // 3. Sync gravity device visual glow
    if let Some(dev) = world_res.0.get_gravity_device(OBJECT_GRAVITY_DEVICE) {
        for mat_handle in gravity_core_query.iter() {
            if let Some(material) = materials.get_mut(&mat_handle.0) {
                if dev.is_active {
                    material.emissive = LinearRgba::new(0.4, 2.5, 3.5, 1.0);
                } else {
                    material.emissive = LinearRgba::new(0.05, 0.2, 0.3, 1.0);
                }
            }
        }
    }
}
