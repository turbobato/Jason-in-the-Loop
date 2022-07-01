use crate::{
    components::*,
    player::{PlayerAnimations, PLAYER_DIMENSIONS, PLAYER_SCALE},
    PLATFORM_MARGIN,
};
use bevy::{
    prelude::*,
};

#[derive(Clone)]
pub enum Actions {
    Jump,
    Left,
    Right,
    Attack,
    Idle,
}
pub struct PlayerLoopPlugin;

impl Plugin for PlayerLoopPlugin {
    fn build(&self, app: &mut App) {
        let clones: Vec<Entity> = Vec::new();
        app.add_system(player_loop_record_system)
            .add_system(loop_movement_system)
            .insert_resource(clones);
    }
}

fn player_loop_record_system(
    animations: Res<PlayerAnimations>,
    mut commands: Commands,
    mut clones: ResMut<Vec<Entity>>,
    kb: Res<Input<KeyCode>>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &Velocity,
            &Acceleration,
            &mut RecordingOn,
            Option<&mut Recording>,
        ),
        With<Player>,
    >,
) {
    let (entity_id, transform, velocity, acceleration, mut recording_on, recording_option) =
        query.get_single_mut().unwrap();
    if kb.just_pressed(KeyCode::K) {
        recording_on.0 = !recording_on.0;
        if !recording_on.0 {
            //remove recording if we pressed K without spawning a clone
            if let Some(ref _recording) = recording_option {
                commands.entity(entity_id).remove::<Recording>();
            }
        }
    }

    if recording_on.0 && kb.just_pressed(KeyCode::L) {
        recording_on.0 = false;
        if let Some(recording) = recording_option {
            commands.entity(entity_id).remove::<Recording>();
            let clone_id = commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: animations.idle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            recording.initial_pos.x,
                            recording.initial_pos.y,
                            recording.initial_pos.z,
                        ),
                        scale: Vec3::splat(PLAYER_SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(TemporalGhost)
                .insert(Velocity {
                    ..Default::default()
                })
                .insert(Acceleration {
                    ..Default::default()
                })
                .insert(Grounded(false))
                .insert(SpriteSize::from(PLAYER_DIMENSIONS))
                .insert(recording.clone())
                .insert(MovingPlatform)
                .insert(Platform {
                    position: recording.initial_pos.clone(),
                    size: Vec2::new(10., PLATFORM_MARGIN),
                })
                .id();
            clones.push(clone_id);
        }
    } else if recording_on.0 {
        let mut buff: Vec<Actions> = Vec::new();
        if kb.pressed(KeyCode::Q) {
            buff.push(Actions::Left);
        } else if kb.pressed(KeyCode::D) {
            buff.push(Actions::Right);
        } else if kb.pressed(KeyCode::J) {
            buff.push(Actions::Attack);
        } else {
            buff.push(Actions::Idle);
        }
        if kb.pressed(KeyCode::Z) {
            buff.push(Actions::Jump);
        }
        if let Some(mut recording) = recording_option {
            recording.recorded_actions.push(buff);
        } else {
            let mut recorded_actions = Vec::new();
            recorded_actions.push(buff);
            commands.entity(entity_id).insert(Recording {
                index: 0,
                initial_pos: transform.translation.clone(),
                initial_acceleration: acceleration.clone(),
                initial_speed: velocity.clone(),
                recorded_actions,
            });
        }
    }

    if kb.just_pressed(KeyCode::I) {
        if let Some(entity_id) = clones.pop() {
            commands.entity(entity_id).despawn();
        }
    }
}

fn loop_movement_system(
    animations: Res<PlayerAnimations>,
    mut query: Query<
        (
            &mut Grounded,
            &mut Velocity,
            &mut Acceleration,
            &mut Handle<TextureAtlas>,
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut Recording,
        ),
        With<TemporalGhost>,
    >,
) {
    for (
        mut grounded,
        mut velocity,
        mut acceleration,
        mut texture_atlas,
        mut transform,
        mut sprite,
        mut recording,
    ) in query.iter_mut()
    {
        if recording.index == 0 {
            transform.translation = recording.initial_pos.clone();
            *acceleration = recording.initial_acceleration.clone();
            *velocity = recording.initial_speed.clone();
        }
        for action in &recording.recorded_actions[recording.index] {
            match action {
                Actions::Left => {
                    velocity.vx = -200.;
                    transform.scale.x = -PLAYER_SCALE;
                    if *texture_atlas != animations.run {
                        *texture_atlas = animations.run.clone();
                    };
                }
                Actions::Right => {
                    velocity.vx = 200.;
                    transform.scale.x = PLAYER_SCALE;
                    if *texture_atlas != animations.run {
                        *texture_atlas = animations.run.clone();
                    };
                }
                Actions::Attack => {
                    velocity.vx = 0.;
                    if *texture_atlas != animations.attack_combo {
                        *texture_atlas = animations.attack_combo.clone();
                        sprite.index = 0;
                    };
                }
                Actions::Jump => {
                    if grounded.0 {
                        velocity.vy = 250.;
                        transform.translation.y += PLATFORM_MARGIN; //this line is to be sure the player gets out of the platform
                        grounded.0 = false;
                    }
                }
                Actions::Idle => {
                    velocity.vx = 0.;
                    if *texture_atlas != animations.idle {
                        *texture_atlas = animations.idle.clone();
                    };
                }
            }
        }
        if velocity.vy < -1. {
            if *texture_atlas != animations.jump_fall {
                *texture_atlas = animations.jump_fall.clone();
                sprite.index = 0;
            }
        } else if velocity.vy > 1. {
            if *texture_atlas != animations.jump {
                *texture_atlas = animations.jump.clone();
                sprite.index = 0;
            }
        }
        recording.index = recording.index + 1;
        recording.index = recording.index % recording.recorded_actions.len();
    }
}
