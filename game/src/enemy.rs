use bevy::prelude::*;
use crate::{components::*, WinSize};

// Les différents chemins des png des animations
const ATTACK_SPRITE_EYE: &str = "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/Attack3.png";
const SPRITE_DIMENSIONS : (f32,f32) = (150., 150.);

const PROJECTILE_SPRITE_EYE: &str = "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/projectile_sprite.png";
const PROJECTILE_SPRITE_DIMENSION : (f32, f32) = (48., 48.);
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_setup)
        .add_system(enemy_movement)
        .add_system(enemy_attack_system);
    }
}

// les différentes animations des ennemis
pub struct EnemyAnimations {
    pub attack: Handle<TextureAtlas>,
    pub projectile: Handle<TextureAtlas>
}

fn enemy_attack_system(mut commands: Commands, animations : Res<EnemyAnimations>, enemy_query : Query<&Transform, With<Enemy>>){
    for &tf in enemy_query.iter(){
        let (x, y) = (tf.translation.x, tf.translation.y);
        commands.spawn_bundle(SpriteSheetBundle{
            texture_atlas: animations.projectile.clone(),
            transform: Transform { 
                translation: Vec3::new(x, y - 25., 1.), 
                ..Default::default()},
        ..Default::default()})
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Velocity{vx: 0., vy: -5.})
        .insert(Projectile)
        .insert(FromEnemy);
    }

}

fn enemy_movement(time: Res<Time>, textures_atlas: Res<Assets<TextureAtlas>>, mut query: Query<(&mut Velocity, &mut Transform, &Handle<TextureAtlas>), Or<(With<Enemy>, With<Projectile>)>>){
    let delta = time.delta_seconds();
    for (velocity, mut transform, texture_atlas) in query.iter_mut(){
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
    }
}
// copier-coller de sprite_sheet avec début de modif
fn enemy_setup(
    mut commands: Commands,
    win_size : Res<WinSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // animation perso
    let texture_handle_perso = asset_server.load(ATTACK_SPRITE_EYE);
    let texture_atlas_perso = TextureAtlas::from_grid(texture_handle_perso, Vec2::new(SPRITE_DIMENSIONS.0, SPRITE_DIMENSIONS.1), 6, 1);
    let texture_atlas_handle_perso = texture_atlases.add(texture_atlas_perso);

    // animation projectile
    let texture_handle_proj = asset_server.load(PROJECTILE_SPRITE_EYE);
    let texture_atlas_proj = TextureAtlas::from_grid(texture_handle_proj, Vec2::new(PROJECTILE_SPRITE_DIMENSION.0, PROJECTILE_SPRITE_DIMENSION.1), 8, 1);
    let texture_atlas_handle_proj = texture_atlases.add(texture_atlas_proj);

    let enemy_animations_ressource = EnemyAnimations {
        attack: texture_atlas_handle_perso.clone(),
        projectile: texture_atlas_handle_proj.clone()
    };
    commands.insert_resource(enemy_animations_ressource);


    commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_perso,
            transform: Transform {
                translation: Vec3::new(0., 0. , 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity{vx: 10., vy: 0.});
}