use crate::{components::*, PLATFORM_MARGIN};
use bevy::{prelude::*,};

pub const RUN_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png";
pub const IDLE_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Idle.png";

pub const ATTACK_COMBO_SPRITE: &str =
    "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_AttackCombo.png";
pub const JUMP_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Jump.png";
pub const JUMP_FALL_SPRITE: &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Fall.png";
pub const TURN_AROUND_SPRITE: &str =
    "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_TurnAround.png";

pub const PLAYER_DIMENSIONS: (f32, f32) = (PLAYER_SCALE * 20., PLAYER_SCALE * 80.); //dimensions for idle sprite
pub const PLAYER_SCALE: f32 = 1.5;
pub const PLAYER_SPAWN: (f32, f32, f32) = (-356., -145. + 90., 1.1); //player spawn coordinates

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
        app.add_startup_system(player_setup)
            .add_system(player_keyboard_event_system)
            .add_system(resize_attack);
    }
}

fn player_setup(
    mut commands: Commands,
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
    let texture_atlas_jf = TextureAtlas::from_grid(jump_fall_sprite, Vec2::new(120.0, 80.0), 3, 1);
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
                translation: Vec3::new(PLAYER_SPAWN.0, PLAYER_SPAWN.1, PLAYER_SPAWN.2),
                scale: Vec3::splat(PLAYER_SCALE),
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
        .insert(Grounded(false))
        .insert(SpriteSize::from(PLAYER_DIMENSIONS))
        .insert(RecordingOn(false))
        .insert(Platform {
            position: Vec3::new(4250., 100. + PLAYER_SPAWN.1, PLAYER_SPAWN.2),
            size: Vec2::new(10., PLATFORM_MARGIN),
        })
        .insert(MovingPlatform)
        .insert(Attack {
            is_attacking: false,
            is_attacked: false,
        })
        .insert(SpriteSizeAttack {
            ..Default::default()
        });
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut query: Query<
        (
            &mut Grounded,
            &mut Velocity,
            &mut Acceleration,
            &mut Handle<TextureAtlas>,
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut Attack,
        ),
        With<Player>,
    >,
) {
    if let Ok((
        mut grounded,
        mut velocity,
        acceleration,
        mut texture_atlas,
        mut transform,
        mut sprite,
        mut attack,
    )) = query.get_single_mut()
    {
        if kb.pressed(KeyCode::Q) && !attack.is_attacking {
            velocity.vx = -200.;
            transform.scale.x = -PLAYER_SCALE;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else if kb.pressed(KeyCode::D) && !attack.is_attacking {
            velocity.vx = 200.;
            transform.scale.x = PLAYER_SCALE;
            if *texture_atlas != animations.run {
                *texture_atlas = animations.run.clone();
            };
        } else if kb.pressed(KeyCode::J) {
            velocity.vx = 0.;
            if *texture_atlas != animations.attack_combo {
                *texture_atlas = animations.attack_combo.clone();
                sprite.index = 0;
                attack.is_attacking = true;
            };
        } else if !attack.is_attacking {
            velocity.vx = 0.;
            if *texture_atlas != animations.idle {
                *texture_atlas = animations.idle.clone();
            };
        }

        if kb.pressed(KeyCode::Z) {
            if grounded.0 {
                velocity.vy = 250.;
                grounded.0 = false;
                attack.is_attacking = false;
            }
        }

        if velocity.vy < -1. {
            if *texture_atlas != animations.jump_fall {
                *texture_atlas = animations.jump_fall.clone();
                sprite.index = 0;
                attack.is_attacking = false;
            }
        } else if velocity.vy > 1. {
            if *texture_atlas != animations.jump {
                *texture_atlas = animations.jump.clone();
                sprite.index = 0;
                attack.is_attacking = false;
            }
        }

        if kb.pressed(KeyCode::Space) {
            respawn(velocity, transform, acceleration);
        }
    }
}

fn respawn(
    mut velocity: Mut<Velocity>,
    mut transform: Mut<Transform>,
    mut acceleration: Mut<Acceleration>,
) {
    (velocity.vx, velocity.vy) = (0., 0.);
    transform.translation = Vec3::new(PLAYER_SPAWN.0, PLAYER_SPAWN.1, PLAYER_SPAWN.2);
    (acceleration.ax, acceleration.ay) = (0., 0.);
}

fn resize_attack(
    mut query_player: Query<
        (
            &Transform,
            &mut Attack,
            &TextureAtlasSprite,
            &mut SpriteSizeAttack,
        ),
        With<Player>,
    >,
) {
    for (transform, mut attack, sprite, mut sprite_size_attack) in query_player.iter_mut() {
        if attack.is_attacking {
            //println!("{}", sprite_size_attack.size[0]);
            if sprite.index == 9 {
                sprite_size_attack.size[0] = 0.;
                sprite_size_attack.size[1] = 0.;
                sprite_size_attack.position[0] = transform.translation.x;
                sprite_size_attack.position[1] = transform.translation.y;
                attack.is_attacking = false;
            } else if sprite.index == 8 {
                sprite_size_attack.size[0] = 29.;
                sprite_size_attack.size[1] = 15.;
                sprite_size_attack.position[0] = transform.translation.x - (15. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (33. * PLAYER_SCALE);
            } else if sprite.index == 7 {
                sprite_size_attack.size[0] = 59.;
                sprite_size_attack.size[1] = 64.;
                sprite_size_attack.position[0] = transform.translation.x + (3. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (32. * PLAYER_SCALE);
            } else if sprite.index == 6 {
                sprite_size_attack.size[0] = 0.;
                sprite_size_attack.size[1] = 0.;
                sprite_size_attack.position[0] = transform.translation.x;
                sprite_size_attack.position[1] = transform.translation.y;
            } else if sprite.index == 5 {
                sprite_size_attack.size[0] = 0.;
                sprite_size_attack.size[1] = 0.;
                sprite_size_attack.position[0] = transform.translation.x;
                sprite_size_attack.position[1] = transform.translation.y;
            } else if sprite.index == 4 {
                sprite_size_attack.size[0] = 63.;
                sprite_size_attack.size[1] = 40.;
                sprite_size_attack.position[0] = transform.translation.x + (18. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (20. * PLAYER_SCALE);
            } else if sprite.index == 3 {
                sprite_size_attack.size[0] = 26.;
                sprite_size_attack.size[1] = 12.;
                sprite_size_attack.position[0] = transform.translation.x + (38. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (34. * PLAYER_SCALE);
            } else if sprite.index == 2 {
                sprite_size_attack.size[0] = 30.;
                sprite_size_attack.size[1] = 45.;
                sprite_size_attack.position[0] = transform.translation.x + (41. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (18. * PLAYER_SCALE);
            } else if sprite.index == 1 {
                sprite_size_attack.size[0] = 38.;
                sprite_size_attack.size[1] = 38.;
                sprite_size_attack.position[0] = transform.translation.x + (37. * PLAYER_SCALE);
                sprite_size_attack.position[1] = transform.translation.y - (21. * PLAYER_SCALE);
            } else if sprite.index == 0 {
                sprite_size_attack.size[0] = 0.;
                sprite_size_attack.size[1] = 0.;
                sprite_size_attack.position[0] = transform.translation.x;
                sprite_size_attack.position[1] = transform.translation.y;
            }
        }
    }
}
