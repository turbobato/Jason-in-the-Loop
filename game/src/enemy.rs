use bevy::prelude::*;
use crate::{components::*, WinSize};

// Les différents chemins des png des animations
const ATTACK_SPRITE_EYE: &str = "textures/monsters/Monster_Creatures_Fantasy(Version 1.3)/Flying eye/Attack3.png";
const SPRITE_DIMENSIONS : (f32,f32) = (150., 150.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_setup);
    }
}

// les différentes animations des ennemis
pub struct EnemyAnimation{
    pub attack: Handle<TextureAtlas>,
}

// copier-coller de sprite_sheet avec début de modif
fn enemy_setup(
    mut commands: Commands,
    win_size : Res<WinSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(ATTACK_SPRITE_EYE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_DIMENSIONS.0, SPRITE_DIMENSIONS.1), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            //transform: Transform::from_scale(Vec3::splat(6.0)),
            transform: Transform {
                translation: Vec3::new(0., 0. , 10.),
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy)
        .insert(Velocity{
            ..Default::default()});
}