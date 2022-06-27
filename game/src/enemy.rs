use crate::{components::*, WinSize};
use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

// Les différents chemins des png des animations
const ATTACK_SPRITE_EYE: &str =
    "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/Attack3.png";
const SPRITE_DIMENSIONS_EYE: (f32, f32) = (150., 150.);
const PROJECTILE_SPRITE_EYE: &str =
    "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/projectile_sprite.png";
const PROJECTILE_SPRITE_EYE_DIMENSION: (f32, f32) = (48., 48.);

const ATTACK_SPRITE_SKETELON: &str =
    "textures/monster2/Skeleton/Attack.png";
const WALK_SPRITE_SKELETON: &str =
"textures/monster2/Skeleton/Walk.png";
const SPRITE_DIMENSIONS_SKELETON: (f32, f32) = (150., 150.);
const PROJECTILE_SPRITE_SKELETON: &str =
    "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Skeleton/sword_sprite.png";
const PROJECTILE_SPRITE_SKELETON_DIMENSION: (f32, f32) = (92., 106.);
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_setup)
            .add_system(eye_movement)
            .add_system(skeleton_movement)
            .add_system(projectile_movement)
           // .add_system_set(
           //     SystemSet::new()
           //         .with_run_criteria(FixedTimestep::step(1.))
           //         .with_system(skeleton_attack_system))
            
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(3.))
                    .with_system(eye_attack_system)
            );
    }
}

// les différentes animations des ennemis
pub struct EnemyAnimations {
    pub attack_eye: Handle<TextureAtlas>,
    pub projectile_eye: Handle<TextureAtlas>,
    pub attack_skeleton: Handle<TextureAtlas>,
    pub walk_skeleton: Handle<TextureAtlas>,
}

// attaque aléatoire
fn enemy_attack_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 120.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}


fn skeleton_movement(
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query_monster: Query<(&mut Velocity, &mut Transform), (With<Skeleton>, Without<Player>)>,
    query_player: Query<&Transform, With<Player>>
) {
    let delta = time.delta_seconds();
    const MARGIN: f32 = 50.;

    let tf_player = query_player.single();

    for (mut velocity, mut transform) in query_monster.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;

        if transform.translation.x <=  tf_player.translation.x - MARGIN
        {
            velocity.vx = 30.; 
            // on est à gauche
        }
        else if transform.translation.x >= tf_player.translation.x + MARGIN {
            velocity.vx = -30.;
            // on est à droite
        }
        else{
            velocity.vx = 0.;
            // on est à côté
        }
    }
}

fn eye_attack_system(
    mut commands: Commands,
    animations: Res<EnemyAnimations>,
    enemy_query: Query<(&Transform, &Velocity), With<Eye>>,
) {
    for (&tf, velocity) in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: animations.projectile_eye.clone(),
                transform: Transform {
                    translation: Vec3::new(x + 50. * velocity.vx.signum(), y - 10., 10.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(Velocity {
                vx: 150. * velocity.vx.signum(),
                vy: 0.,
            })
            .insert(Projectile)
            .insert(FromEnemy);
    }
}

// mouvements des projectiles avec auto-despawn
fn projectile_movement(
    time: Res<Time>,
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Projectile>,
    >,
) {
    let delta = time.delta_seconds();
    const MARGIN: f32 = 50.;
    for (entity, velocity, mut transform, sprite) in query.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;

        if sprite.index == 7 {
            commands.entity(entity).despawn();
        } else if transform.translation.x <= -win_size.win_h / 2.0 - MARGIN
            || transform.translation.x >= win_size.win_h / 2.0 + MARGIN
        {
            commands.entity(entity).despawn();
        }
    }
}

// mouvement des ennemis
fn eye_movement(
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Eye>>,
) {
    let delta = time.delta_seconds();
    const MARGIN: f32 = 50.;
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;

        if transform.translation.x <= -win_size.win_w / 2. + MARGIN
            || transform.translation.x >= win_size.win_w / 2. - MARGIN
        {
            velocity.vx = -velocity.vx;
            transform.scale.x *= -1.;
        }
    }
}

// copier-coller de sprite_sheet avec début de modif
fn enemy_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // animation perso
    let texture_handle_perso = asset_server.load(ATTACK_SPRITE_EYE);
    let texture_atlas_perso = TextureAtlas::from_grid(
        texture_handle_perso,
        Vec2::new(SPRITE_DIMENSIONS_EYE.0, SPRITE_DIMENSIONS_EYE.1),
        6,
        1,
    );
    let texture_atlas_handle_perso = texture_atlases.add(texture_atlas_perso);

    // animation projectile
    let texture_handle_proj = asset_server.load(PROJECTILE_SPRITE_EYE);
    let texture_atlas_proj = TextureAtlas::from_grid(
        texture_handle_proj,
        Vec2::new(
            PROJECTILE_SPRITE_EYE_DIMENSION.0,
            PROJECTILE_SPRITE_EYE_DIMENSION.1,
        ),
        8,
        1,
    );
    let texture_atlas_handle_proj = texture_atlases.add(texture_atlas_proj);

    let texture_handle_skeleton = asset_server.load(ATTACK_SPRITE_SKETELON);
    let texture_atlas_skeleton = TextureAtlas::from_grid(
        texture_handle_skeleton,
        Vec2::new(SPRITE_DIMENSIONS_SKELETON.0, SPRITE_DIMENSIONS_SKELETON.1),
        6,
        1,
    );
    let texture_atlas_handle_skeleton = texture_atlases.add(texture_atlas_skeleton);

    let texture_handle_skeleton_walk = asset_server.load(WALK_SPRITE_SKELETON);
    let texture_atlas_skeleton_walk = TextureAtlas::from_grid(
        texture_handle_skeleton_walk,
        Vec2::new(SPRITE_DIMENSIONS_SKELETON.0, SPRITE_DIMENSIONS_SKELETON.1),
        4,
        1,
    );
    let texture_atlas_handle_skeleton_walk = texture_atlases.add(texture_atlas_skeleton_walk);

    let enemy_animations_ressource = EnemyAnimations {
        attack_eye: texture_atlas_handle_perso.clone(),
        projectile_eye: texture_atlas_handle_proj.clone(),
        attack_skeleton: texture_atlas_handle_skeleton.clone(),
        walk_skeleton: texture_atlas_handle_skeleton_walk.clone(),
    };
    commands.insert_resource(enemy_animations_ressource);

    // eye spawn avec la sheet attaque
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_perso,
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity { vx: 50., vy: 0. })
        .insert(Eye);

    // squelette spawn avec la sheet walk
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_skeleton_walk,
            transform: Transform {
                translation: Vec3::new(0., -100., 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity { vx: 0., vy: 0. })
        .insert(Skeleton);
}
