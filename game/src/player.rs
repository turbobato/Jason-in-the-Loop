use crate::{components::*,GroundLevel};

use bevy::{prelude::*, transform};

const RUN_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png";
const IDLE_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Idle.png";
pub struct PlayerPlugin;

// ressource for player animations

pub struct PlayerAnimations {
    pub idle: Handle<TextureAtlas>,
    pub run: Handle<TextureAtlas>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_setup)
        .add_system(player_keyboard_event_system);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ground_level: Res<GroundLevel>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_sprite = asset_server.load(IDLE_SPRITE);
    let run_sprite = asset_server.load(RUN_SPRITE);
    let texture_atlas_running = TextureAtlas::from_grid(run_sprite, Vec2::new(120., 80.), 10, 1);
    let texture_atlas_handle_running = texture_atlases.add(texture_atlas_running);
    let texture_atlas_idle = TextureAtlas::from_grid(idle_sprite, Vec2::new(120., 80.), 10, 1);
    let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
    let idle = texture_atlas_handle_idle.clone();
    let animations_ressource = PlayerAnimations {
        run: texture_atlas_handle_running,
        idle,
    };
    commands.insert_resource(animations_ressource);
    let level = ground_level.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_idle,
            transform: Transform {
                translation: Vec3::new(0., level, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(Velocity {
            vx: 1.,
            ..Default::default()
        });
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    ground_level: Res<GroundLevel>,
    animations: Res<PlayerAnimations>,
    mut query: Query<
        (
            &mut Velocity,
            &mut Handle<TextureAtlas>,
            &mut Transform
        ),
        With<Player>,
    >,
) {
    if let Ok((mut velocity,mut texture_atlas,mut transform)) = query.get_single_mut() {
        if kb.pressed(KeyCode::Left) {
            velocity.vx = -100.;
            transform.scale.x = -1.;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        }
        else if kb.pressed(KeyCode::Right) {
            velocity.vx = 100.;
            transform.scale.x = 1.;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        }
        else {
            velocity.vx = 0.;
            if *texture_atlas != animations.idle {
                *texture_atlas = animations.idle.clone();
            };
        }

        if kb.pressed(KeyCode::Space){
            velocity.vy = 100.;
        }
        else if transform.translation.y > ground_level.0 {
            velocity.vy = -50.;
        }
        else {
            velocity.vy = 0.;
        }
    }
}
