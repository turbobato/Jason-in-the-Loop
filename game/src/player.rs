use crate::{components::*, WinSize, GROUND_LEVEL, PLATFORM_MARGIN};
use bevy::{prelude::*, transform};

const RUN_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png";
const IDLE_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Idle.png";

const ATTACK_COMBO_SPRITE: &str =
    "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_AttackCombo.png";
const JUMP_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Jump.png";
const JUMP_FALL_SPRITE: &str =
    "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_JumpFallInbetween.png";
const TURN_AROUND_SPRITE: &str =
    "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_TurnAround.png";

const PLAYER_DIMENSIONS: (f32, f32) = (30., 80.); //dimensions for idle sprite

pub struct PlayerPlugin;

// ressource for player animations

pub struct PlayerAnimations {
    pub idle: Handle<TextureAtlas>,
    pub run: Handle<TextureAtlas>,
    pub attack_combo: Handle<TextureAtlas>,
    pub jump: Handle<TextureAtlas>,
    pub jump_fall: Handle<TextureAtlas>,
    pub turn_around: Handle<TextureAtlas>,
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
    let run_sprite = asset_server.load(RUN_SPRITE);
    let texture_atlas_running = TextureAtlas::from_grid(run_sprite, Vec2::new(120., 80.), 10, 1);
    let texture_atlas_handle_running = texture_atlases.add(texture_atlas_running);

    let idle_sprite = asset_server.load(IDLE_SPRITE);
    let texture_atlas_idle = TextureAtlas::from_grid(idle_sprite, Vec2::new(120., 80.), 10, 1);
    let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
    let idle = texture_atlas_handle_idle.clone();

    let attack_combo_sprite: Handle<Image> = asset_server.load(ATTACK_COMBO_SPRITE);
    let texture_atlas_a1 =
        TextureAtlas::from_grid(attack_combo_sprite, Vec2::new(120.0, 80.0), 10, 1);
    let texture_atlas_attack_combo = texture_atlases.add(texture_atlas_a1);

    let jump_sprite: Handle<Image> = asset_server.load(JUMP_SPRITE);
    let texture_atlas_j = TextureAtlas::from_grid(jump_sprite, Vec2::new(120.0, 80.0), 3, 1);
    let texture_atlas_jump = texture_atlases.add(texture_atlas_j);

    let jump_fall_sprite: Handle<Image> = asset_server.load(JUMP_FALL_SPRITE);
    let texture_atlas_jf = TextureAtlas::from_grid(jump_fall_sprite, Vec2::new(120.0, 80.0), 2, 1);
    let texture_atlas_jump_fall = texture_atlases.add(texture_atlas_jf);

    let turn_around_sprite: Handle<Image> = asset_server.load(TURN_AROUND_SPRITE);
    let texture_atlas_ta =
        TextureAtlas::from_grid(turn_around_sprite, Vec2::new(120.0, 80.0), 3, 1);
    let texture_atlas_turn_around = texture_atlases.add(texture_atlas_ta);

    let animations_ressource = PlayerAnimations {
        run: texture_atlas_handle_running,
        idle,
        attack_combo: texture_atlas_attack_combo,
        jump: texture_atlas_jump,
        jump_fall: texture_atlas_jump_fall,
        turn_around: texture_atlas_turn_around,
    };
    commands.insert_resource(animations_ressource);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_idle,
            transform: Transform {
                translation: Vec3::new(0., GROUND_LEVEL + PLAYER_DIMENSIONS.1 / 2., 1.),
                scale: Vec3::splat(1.5),
                ..Default::default()
            },
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
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut grounded, mut velocity, mut texture_atlas, mut transform, mut sprite)) =
        query.get_single_mut()
    {
        if kb.pressed(KeyCode::Q) {
            velocity.vx = -100.;
            transform.scale.x = -1.5;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else if kb.pressed(KeyCode::D) {
            velocity.vx = 100.;
            transform.scale.x = 1.5;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else if kb.pressed(KeyCode::J) {
            //velocity.vx = 0.;
            if *texture_atlas != animations.attack_combo {
                *texture_atlas = animations.attack_combo.clone();
                sprite.index = 0;
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
