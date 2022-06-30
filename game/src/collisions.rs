use crate::*;
use bevy::prelude::*;

pub const COLLISION_MARGIN: f32 = 4.; //margin for collisions

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_with_platform);
    }
}

fn collision_with_platform(
    query_platforms: Query<&Platform>,
    mut query: Query<
        (&mut Grounded, &mut Transform, &SpriteSize, &mut Velocity),
        Or<(With<Player>, With<Skeleton>)>,
    >,
) {
    for (mut grounded, mut transform, sprite_size, mut velocity) in query.iter_mut() {
        for platform in query_platforms.iter() {
            let entity_position = &mut transform.translation;
            let entity_size = sprite_size.0;
            let platform_position = platform.position;
            let platform_size = platform.size;
            if let Some(collision) = collide(
                *entity_position,
                entity_size + COLLISION_MARGIN, //add margin to make sure that when player is on ground, there is collision
                platform_position,
                platform_size + COLLISION_MARGIN,
            ) {
                match collision {
                    Collision::Top => {
                        if velocity.vy < 1. {
                            grounded.0 = true;
                            velocity.vy = 0.;
                            entity_position.y =
                                platform_position.y + platform_size.y / 2. + entity_size.y / 2.;
                            //set player on the platform to avoid float precision issues
                            break;
                        }
                    }
                    Collision::Left => {
                        if velocity.vx > 0. {
                            velocity.vx = 0.;
                            entity_position.x =
                                platform_position.x - platform_size.x / 2. - entity_size.x / 2.;
                        }
                    }
                    Collision::Right => {
                        if velocity.vx < 0. {
                            velocity.vx = 0.;
                            entity_position.x =
                                platform_position.x + platform_size.x / 2. + entity_size.x / 2.;
                        }
                    }
                    _ => (),
                };
            } else {
                grounded.0 = false;
            }
        }
    }
}
