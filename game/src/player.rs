use bevy::prelude::*;
use crate::{components::*, GroundLevel};

const CROUCH_SPRITE : &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Crouch.png";
/* 
struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app : &mut App){
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(player_spawn_system),
        );
    }
    
}

fn player_spawn_system(mut commands : Commands, asset_server : AssetServer, ground_level : Res<GroundLevel>){
    let player_sprite = asset_server.load(CROUCH_SPRITE);
    let level = ground_level.0;
    commands.spawn_bundle(SpriteBundle{
        texture : player_sprite,
        transform : Transform {
            translation : Vec3::new(0.,level,0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
*/