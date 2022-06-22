use bevy::prelude::*;
use crate::{components::*, GroundLevel};

const RUN_SPRITE : &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png";
const IDLE_SPRITE : &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Idle.png";
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app : &mut App){
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_setup);
    }
    
}

fn player_setup(mut commands : Commands, asset_server : Res<AssetServer>, ground_level : Res<GroundLevel>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let idle_sprite = asset_server.load(IDLE_SPRITE);
    let run_sprite = asset_server.load(RUN_SPRITE);
    let texture_atlas_running = TextureAtlas::from_grid(
        run_sprite,
        Vec2::new(120.,80.),
        10,
        1,
    );
    let texture_atlas_handle_running = texture_atlases.add(texture_atlas_running);
    let texture_atlas_idle = TextureAtlas::from_grid(
        idle_sprite,
        Vec2::new(120.,80.),
        10,
        1
    );
    let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
    let level = ground_level.0;
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas : texture_atlas_handle_running,
        transform : Transform {
            translation : Vec3::new(0.,level,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}
