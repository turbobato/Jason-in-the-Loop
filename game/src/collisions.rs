use bevy::prelude::*;
use crate::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app : &mut App){
        app.add_system(collision_with_platform);
    }
}

fn collision_with_platform(
    mut query: Query<(&mut Grounded, &mut Transform, &SpriteSize), With<Player>>,
    query_platforms: Query<&Platform>,
) {
    for (mut grounded, mut transform, sprite_size) in query.iter_mut() {
        for platform in query_platforms.iter() {
            let entity_position = &mut transform.translation;
            let entity_size = sprite_size.0;
            let platform_position = platform.position;
            let platform_size = platform.size;
            if let Some(collision) = collide(
                *entity_position,
                entity_size,
                platform_position,
                platform_size,
            ) {
                match collision {
                    Collision::Top => {
                        grounded.0 = true;
                        entity_position.y = platform_position.y + entity_size.y / 2.;
                    }
                    _ => (),
                }
            }
        }
    }
}