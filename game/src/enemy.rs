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
            .add_system(skeleton_follow_player)
            .add_system(projectile_movement)
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

// renvoie vrai si le perso est sur la plateforme (bon intervalle de x et de y)
fn is_on_plat(x1_plat:f32, x2_plat:f32, y_plat:f32, x_perso: f32, y_perso: f32) -> bool {
    const MARGIN_MIN: f32 = 0.; // on est bien sur la plat si y_monster > y_plat + MARGIN8MIN
    const MARGIN_MAX: f32 = 80.; // on est bien sur cette plateforme et pas celle du haut si y_monster < y_plat + MARGIN_MAX
    
    return y_perso - y_plat > MARGIN_MIN
        && (y_perso - y_plat).abs() < MARGIN_MAX
        && x1_plat <= x_perso
        && x_perso <= x2_plat;
}

fn is_plat_below(x1_plat: f32, x2_plat: f32, y_plat: f32, x_perso: f32, y_perso: f32) -> bool {
    const MARGIN_MIN: f32 = 0.;
    return y_perso - y_plat > MARGIN_MIN && x1_plat <= x_perso && x_perso <= x2_plat;
}

// attaque aléatoire
fn enemy_attack_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 120.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

// le squelette suit le player selon
fn skeleton_follow_player(
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
    query_plat: Query<&Platform>,
    enemy_animations: Res<EnemyAnimations>,
) {
    const MARGIN_WALK: f32 = 40.;
    const MARGIN_IN: f32 = 80.; // portée de l'attaque
    const MARGIN_OUT: f32 = 400.; // portée de poursuite
    const MARGIN_Y: f32 = 31.; // erreur en y

    let tf_player = query_player.single();
    let (x_player, y_player) = (tf_player.translation.x, tf_player.translation.y);

    for (
        mut velocity,
        mut tf_monster,
        mut texture_atlas,
        mut sprite,
        mut acceleration,
        mut grounded,
    ) in query_monster.iter_mut()
    {
        let (x_monster, y_monster) = (tf_monster.translation.x, tf_monster.translation.y);
       
        let mut x2_plat_monster = 0.;
        let mut x1_plat_monster = 0.;
        let mut y_plat_monster= 0;

        let mut x2_plat_player = 0.;
        let mut x1_plat_player = 0.;
        let mut y_plat_player = 0.;

        // se tourne vers le personnage
        if x_monster <= x_player {
            tf_monster.scale.x = 1.;
        } else {
            tf_monster.scale.x = -1.;
        }

        let mut same_plat = false;
        let mut is_on_a_plat_up = false;

        for platform in query_plat.iter() {
            let x1 = platform.position.x - platform.size.x / 2.;
            let x2 = platform.position.x + platform.size.x / 2.;
            let y = platform.position.y + platform.size.y / 2.;

             // on prend les coordonnées de la platforme du monstre
            if is_on_plat(x1, x2, y, x_player, y_player){
                x1_plat_player = x1;
                x2_plat_player = x2;
                y_plat_player = y;
            }
        
             if is_on_plat(x1, x2, y, x_monster, y_monster){
               x1_plat_monster = x1;
                x2_plat_monster = x2;
              //  y_plat_monster = y;
              //let platform_monster = platform;
            }
    
            if is_on_plat(x1, x2, y, x_player, y_player) && is_on_plat(x1, x2, y, x_monster, y_monster){
                same_plat = true;
                if (x_monster - x_player).abs() < MARGIN_IN {
                    velocity.vx = 0.;
                    if *texture_atlas != enemy_animations.attack_skeleton {
                        *texture_atlas = enemy_animations.attack_skeleton.clone();
                        sprite.index = 0;
                  } // attaque
                }
                else if (x_monster - x_player).abs() > MARGIN_IN && (x_monster - x_player).abs() < MARGIN_OUT{
                    if x_monster > x_player {
                        velocity.vx = -30.;
                    } else {
                        velocity.vx = 30.;
                    }
                
                    if *texture_atlas != enemy_animations.walk_skeleton {
                        *texture_atlas = enemy_animations.walk_skeleton.clone();
                        sprite.index = 0;
                    }
                // walk
                }
                else{
                    velocity.vx = 0.;
                    if *texture_atlas != enemy_animations.idle_skeleton {
                        *texture_atlas = enemy_animations.idle_skeleton.clone();
                        sprite.index = 0;
                    }
                // idle si meme plat mais loin
                }
            }
        } // fin de la boucle des plat
    
    // On peut descendre si on est juste au dessus de la plateforme player
    if !same_plat {
        if is_plat_below(x1_plat_player, x2_plat_player, y_plat_player, x_monster, y_monster){
           is_on_a_plat_up = true;
        }

        if x_player > x_monster && (x_player-x_monster).abs() < MARGIN_OUT {
            // si il y a une plateforme en dessous ou si on n'est pas au bord on marche
            
            if is_on_a_plat_up || (x_monster - x2_plat_monster).abs() > MARGIN_WALK{
                velocity.vx = 30.;
                if *texture_atlas != enemy_animations.walk_skeleton {
                    *texture_atlas = enemy_animations.walk_skeleton.clone();
                    sprite.index = 0;
                }
            }
            else{
                velocity.vx = 0.;
                if *texture_atlas != enemy_animations.idle_skeleton {
                    *texture_atlas = enemy_animations.idle_skeleton.clone();
                    sprite.index = 0;
                }
            }
        }
        else if x_player < x_monster && (x_player-x_monster).abs() < MARGIN_OUT{
            if is_on_a_plat_up || (x_monster - x1_plat_monster).abs() > MARGIN_WALK {
                velocity.vx = -30.;
                if *texture_atlas != enemy_animations.walk_skeleton {
                    *texture_atlas = enemy_animations.walk_skeleton.clone();
                    sprite.index = 0;
                }
            }
            else{
                velocity.vx = 0.;
                if *texture_atlas != enemy_animations.idle_skeleton {
                    *texture_atlas = enemy_animations.idle_skeleton.clone();
                    sprite.index = 0;
                }
            }
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
fn eye_movement_2(
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Velocity, &mut Transform), With<Eye>>,
) {
    let frame_time = 1. / 60.;
    let now = time.seconds_since_startup() as f32;
    const MARGIN: f32 = 50.;

    // mouvmement circulaire des ennemis
    for (entity, velocity, mut transform) in query.iter_mut() {
        let (x_pivot, y_pivot) = (300., 0.);
        let (x_radius, y_radius) = (70., 70.);
        let max_distance = frame_time * velocity.vx;
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);
        //On peut changer le sens
        let dir = 1;

        let angle = velocity.vx * frame_time * now % 360. / PI;

        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

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
                translation: Vec3::new(-400., 0., 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity { vx: 0., vy: 0. })
        .insert(Skeleton)
        .insert(Grounded(true))
        .insert(Acceleration {
            ..Default::default()
        })
        .insert(SpriteSize(Vec2::new(27., 60.)));
}
