use crate::{components::*, WinSize};
use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};
pub const PI: f32 = 3.141592653589793238462643;

// Les différents chemins des png des animations
const ATTACK_SPRITE_EYE: &str =
    "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/Attack3.png";
const SPRITE_DIMENSIONS_EYE: (f32, f32) = (150., 150.);
const PROJECTILE_SPRITE_EYE: &str =
    "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/projectile_sprite.png";
const PROJECTILE_SPRITE_EYE_DIMENSION: (f32, f32) = (48., 48.);

const ATTACK_SPRITE_SKETELON: &str = "textures/monster2/Skeleton/Attack.png";
const WALK_SPRITE_SKELETON: &str = "textures/monster2/Skeleton/Walk.png";
const IDLE_SPRITE_SKELETON: &str = "textures/monster2/Skeleton/Idle.png";
const SPRITE_DIMENSIONS_SKELETON: (f32, f32) = (150., 150.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_setup)
            .add_system(eye_movement_2)
            .add_system(skeleton_movement)
            .add_system(projectile_movement)
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(1.))
            //         .with_system(skeleton_attack_system))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(3.))
                    .with_system(eye_attack_system),
            );
    }
}

// les différentes animations des ennemis
pub struct EnemyAnimations {
    pub attack_eye: Handle<TextureAtlas>,
    pub projectile_eye: Handle<TextureAtlas>,
    pub attack_skeleton: Handle<TextureAtlas>,
    pub walk_skeleton: Handle<TextureAtlas>,
    pub idle_skeleton: Handle<TextureAtlas>,
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

    mut query_monster: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut Acceleration,
            &Grounded,
        ),
        (With<Skeleton>, Without<Player>),
    >,
    query_player: Query<&Transform, With<Player>>,
    enemy_animations: Res<EnemyAnimations>,
) {
    let delta = time.delta_seconds();
    const MARGIN_IN: f32 = 80.;
    const MARGIN_OUT: f32 = 300.;

    let tf_player = query_player.single();

    for (mut velocity, mut transform, mut texture_atlas, mut sprite, mut acceleration, mut grounded) in query_monster.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
        velocity.vx += acceleration.ax * delta;
        velocity.vy += acceleration.ay * delta;
        if grounded.0 {
            acceleration.ay = 0.;
        } else {
            acceleration.ay = -100.;
        }

        if transform.translation.x <= tf_player.translation.x - MARGIN_IN
            && transform.translation.x > tf_player.translation.x - MARGIN_OUT
        {
            if *texture_atlas != enemy_animations.walk_skeleton {
                *texture_atlas = enemy_animations.walk_skeleton.clone();
                sprite.index = 0;
            }

            if velocity.vx >= 0. {
                transform.scale.x = 1.;
            }

            velocity.vx = 30.;
            // on est à gauche
        } else if transform.translation.x >= tf_player.translation.x + MARGIN_IN
            && transform.translation.x < tf_player.translation.x + MARGIN_OUT
        {
            // on est à droite
            if *texture_atlas != enemy_animations.walk_skeleton {
                *texture_atlas = enemy_animations.walk_skeleton.clone();
                sprite.index = 0;
            }

            if velocity.vx >= 0. {
                transform.scale.x = -1.;
            }
            velocity.vx = -30.;
        } else if transform.translation.x >= tf_player.translation.x - MARGIN_IN
            && transform.translation.x <= tf_player.translation.x + MARGIN_IN
        {
            velocity.vx = 0.;
            if *texture_atlas != enemy_animations.attack_skeleton {
                *texture_atlas = enemy_animations.attack_skeleton.clone();
                sprite.index = 0;
            }
            // on est à côté
        } else {
            velocity.vx = 0.;
            if *texture_atlas != enemy_animations.idle_skeleton {
                *texture_atlas = enemy_animations.idle_skeleton.clone();
                sprite.index = 0;
            }
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
                    translation: Vec3::new(x + 50. * velocity.vx.signum(), y - 10., 2.),
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
fn eye_movement_2(time: Res<Time>, win_size: Res<WinSize>,mut query: Query<(Entity, &mut Velocity, &mut Transform), With<Eye>>){
    let frame_time = 1./60.;
    let now = time.seconds_since_startup() as f32; 
    const MARGIN : f32 = 50.;
    
    // mouvmement circulaire des ennemis
    for (entity, velocity, mut transform) in query.iter_mut(){
        let (x_pivot, y_pivot) = (500.,0.);
		let (x_radius, y_radius) = (70.,70.);
        let max_distance = frame_time * velocity.vx;
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);
	//On peut changer le sens
		let dir = 1;

		let angle = velocity.vx*frame_time*now%360./PI;

		let x_dst = x_radius * angle.cos() + x_org;
		let y_dst = y_radius * angle.sin() + y_org;

		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let distance = (dx * dx + dy * dy).sqrt();
		let distance_ratio = if distance != 0. { max_distance / distance } else { 0. };

		let x = x_org - dx * distance_ratio;
		let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
		let y = y_org - dy * distance_ratio;
		let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

            let translation = &mut transform.translation;
		(translation.x, translation.y) = (x, y);
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
    // animation eye
    let texture_handle_perso = asset_server.load(ATTACK_SPRITE_EYE);
    let texture_atlas_perso = TextureAtlas::from_grid(
        texture_handle_perso,
        Vec2::new(SPRITE_DIMENSIONS_EYE.0, SPRITE_DIMENSIONS_EYE.1),
        6,
        1,
    );
    let texture_atlas_handle_perso = texture_atlases.add(texture_atlas_perso);

    // animation projectile eye
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

    // animation attaque squelette
    let texture_handle_skeleton = asset_server.load(ATTACK_SPRITE_SKETELON);
    let texture_atlas_skeleton = TextureAtlas::from_grid(
        texture_handle_skeleton,
        Vec2::new(SPRITE_DIMENSIONS_SKELETON.0, SPRITE_DIMENSIONS_SKELETON.1),
        8,
        1,
    );
    let texture_atlas_handle_skeleton = texture_atlases.add(texture_atlas_skeleton);

    // animation walk squelette
    let texture_handle_skeleton_walk = asset_server.load(WALK_SPRITE_SKELETON);
    let texture_atlas_skeleton_walk = TextureAtlas::from_grid(
        texture_handle_skeleton_walk,
        Vec2::new(SPRITE_DIMENSIONS_SKELETON.0, SPRITE_DIMENSIONS_SKELETON.1),
        4,
        1,
    );
    let texture_atlas_handle_skeleton_walk = texture_atlases.add(texture_atlas_skeleton_walk);

    let texture_handle_skeleton_idle = asset_server.load(IDLE_SPRITE_SKELETON);
    let texture_atlas_skeleton_idle = TextureAtlas::from_grid(
        texture_handle_skeleton_idle,
        Vec2::new(SPRITE_DIMENSIONS_SKELETON.0, SPRITE_DIMENSIONS_SKELETON.1),
        4,
        1,
    );
    let texture_atlas_handle_skeleton_idle = texture_atlases.add(texture_atlas_skeleton_idle);

    let enemy_animations_ressource = EnemyAnimations {
        attack_eye: texture_atlas_handle_perso.clone(),
        projectile_eye: texture_atlas_handle_proj.clone(),
        attack_skeleton: texture_atlas_handle_skeleton.clone(),
        walk_skeleton: texture_atlas_handle_skeleton_walk.clone(),
        idle_skeleton: texture_atlas_handle_skeleton_idle.clone(),
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
        .insert(Velocity { vx: 100., vy: 0. })
        .insert(Eye);

    // squelette spawn avec la sheet walk
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_skeleton_idle,
            transform: Transform {
                translation: Vec3::new(-200., 0., 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity { vx: 0., vy: 0. })
        .insert(Skeleton)
        .insert(Grounded(true))
        .insert(Acceleration{..Default::default()})
        .insert(SpriteSize(Vec2::new(27.,60.)));
}
