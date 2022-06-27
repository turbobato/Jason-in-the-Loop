use crate::{components::*, WinSize, GROUND_LEVEL, PLATFORM_MARGIN};
use bevy::{prelude::*, transform};

const RUN_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png";
const IDLE_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Idle.png";
const PLAYER_DIMENSIONS: (f32, f32) = (30., 80.); //dimensions for idle sprite
pub struct PlayerPlugin;

// ressource for player animations

pub struct PlayerAnimations {
    pub run: Handle<TextureAtlas>,
    pub idle: Handle<TextureAtlas>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_setup)
            .add_system(player_keyboard_event_system);
    }
}

fn player_setup(
    mut commands: Commands,
    win_size: Res<WinSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_sprite = asset_server.load(IDLE_SPRITE);
    let run_sprite = asset_server.load(RUN_SPRITE);
    let texture_atlas_running = TextureAtlas::from_grid(run_sprite, Vec2::new(SPRITE_DIMENSIONS.0, SPRITE_DIMENSIONS.1), 10, 1);
    let texture_atlas_handle_running = texture_atlases.add(texture_atlas_running);
    let texture_atlas_idle = TextureAtlas::from_grid(idle_sprite, Vec2::new(SPRITE_DIMENSIONS.0, SPRITE_DIMENSIONS.1), 10, 1);
    let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
    let idle = texture_atlas_handle_idle.clone();
    let animations_ressource = PlayerAnimations {
        run: texture_atlas_handle_running,
        idle,
    };
    commands.insert_resource(animations_ressource);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_idle,
            transform: Transform {
                translation: Vec3::new(0.,level + SPRITE_DIMENSIONS.1/2., 1.),
                ..Default::default()},
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(Velocity {
            ..Default::default()
        })
        .insert(Acceleration {
            ..Default::default()
        })
        .insert(Grounded(true))
        .insert(SpriteSize::from(PLAYER_DIMENSIONS));
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut query: Query<
        (
            &mut Grounded,
            &mut Velocity,
            &mut Handle<TextureAtlas>,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut grounded, mut velocity, mut texture_atlas, mut transform)) =
        query.get_single_mut()
    {
        if kb.pressed(KeyCode::Q) {
            velocity.vx = -100.;
            transform.scale.x = -1.;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else if kb.pressed(KeyCode::D) {
            velocity.vx = 100.;
            transform.scale.x = 1.;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else {
            velocity.vx = 0.;
            if *texture_atlas != animations.idle {
                *texture_atlas = animations.idle.clone();
            };
        }

        if kb.pressed(KeyCode::Z) && grounded.0 {
            velocity.vy = 100.;
            transform.translation.x += PLATFORM_MARGIN; //this line is to be sure the player gets out of the platform
            grounded.0 = false;
        }
    }
}
