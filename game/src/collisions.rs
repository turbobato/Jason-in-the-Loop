use crate::*;
use bevy::prelude::*;

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
                entity_size,
                platform_position,
                platform_size,
            ) {
                match collision {
                    Collision::Top => {
                        if velocity.vy <= 1. {
                            grounded.0 = true;
                            velocity.vy = 0.;
                            entity_position.y =
                                platform_position.y + platform_size.y / 2. + entity_size.y / 2.;
                            //set player on the platform to avoid float precision issues
                            break;
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
