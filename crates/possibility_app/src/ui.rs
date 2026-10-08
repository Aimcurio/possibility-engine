use bevy::prelude::*;
use possibility_sim::{OBJECT_BOWL, OBJECT_GRAVITY_DEVICE};

use crate::app::{ArbitratorResource, SimWorldResource};
use crate::player_input::SelectedAvatar;

#[derive(Component)]
pub struct HudText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        HudText,
        Text::new("Initializing The Possibility Engine HUD..."),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 0.98)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(16.0),
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.10, 0.15, 0.85)),
    ));
}

pub fn update_hud(
    selected: Res<SelectedAvatar>,
    world_res: Res<SimWorldResource>,
    arb_res: Res<ArbitratorResource>,
    mut query: Query<&mut Text, With<HudText>>,
) {
    let active_id = selected.0;
    let mode = arb_res
        .0
        .get_mode(active_id)
        .map(|m| m.name())
        .unwrap_or("Unknown");
    let goal = arb_res.0.get_goal(active_id);
    let avatar_rec = world_res.0.get_avatar(active_id);

    let avatar_name = avatar_rec.map(|a| a.name.as_str()).unwrap_or("Avatar");
    let held_obj = avatar_rec
        .and_then(|a| a.grasp.held_object)
        .map(|id| format!("Object #{:?}", id.0))
        .unwrap_or_else(|| "None".to_string());

    let bowl = world_res.0.get_container(OBJECT_BOWL);
    let (cereal_g, milk_ml, is_cereal_ready) = if let Some(b) = bowl {
        (b.cereal_grams, b.milk_ml, b.is_prepared_cereal())
    } else {
        (0.0, 0.0, false)
    };

    let grav_device = world_res.0.get_gravity_device(OBJECT_GRAVITY_DEVICE);
    let grav_active = grav_device.map(|g| g.is_active).unwrap_or(false);

    let hud_string = format!(
        "===============================================================\n\
         THE POSSIBILITY ENGINE — 3D INTERACTIVE SANDBOX\n\
         ===============================================================\n\
         [ACTIVE AVATAR]: {} (ID: {}) | Holding: {}\n\
         [CONTROL MODE ]: {}\n\
         [CURRENT GOAL ]: {:?}\n\
         ---------------------------------------------------------------\n\
         LABORATORY SENSORS:\n\
         - Bowl Contents   : {:.1}g Cereal | {:.1}ml Milk  [Ready: {}]\n\
         - Graviton Field  : {}\n\
         ---------------------------------------------------------------\n\
         KEYBOARD CONTROLS:\n\
         [1, 2, 3] Select Avatar (Alpha / Beta / Gamma)\n\
         [H] Human  [A] Autonomous AI  [D] Directed AI  [C] Copilot\n\
         [W, A, S, D] Move Active Avatar | [Space] Pick Up / Place\n\
         [F1] Run Scenario A (Cereal Prep)\n\
         [F2] Run Scenario B (Artificial Gravity)\n\
         [F3] Run Scenario C (Control Handoff Demo)\n\
         [R]  Reset World State\n\
         ===============================================================",
        avatar_name,
        active_id.0,
        held_obj,
        mode,
        goal,
        cereal_g,
        milk_ml,
        if is_cereal_ready { "YES" } else { "NO" },
        if grav_active {
            "ACTIVE (Generating radial attraction)"
        } else {
            "STANDBY (Zero field force)"
        },
    );

    for mut text in query.iter_mut() {
        text.0 = hud_string.clone();
    }
}
