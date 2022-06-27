mod components;
mod platforms;
mod player;

use bevy::{
    math::{const_vec3, Vec3Swizzles},
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::{collide, Collision},
};
use components::*;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;

const BACKGROUND: &str = "textures/forest/Free Pixel Art Forest/Preview/Background.png";
const BACKGROUND_LAYER1: &str = "textures/oak_woods_v1.0/background/background_layer_1.png";
const BACKGROUND_LAYER2: &str = "textures/oak_woods_v1.0/background/background_layer_2.png";
const BACKGROUND_LAYER3: &str = "textures/oak_woods_v1.0/background/background_layer_3.png";

const BACKGROUND_DIM: (f32, f32) = (640., 360.);

const SPRITE_SCALE: f32 = 2.;
const MARGIN: f32 = 1.;

struct WinSize {
    win_h: f32,
    win_w: f32,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) //prevent blurry sprites
        .insert_resource(WindowDescriptor {
            title: "ProjetX".to_string(),
            width: 640.0,
            height: 360.0,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformsPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(movement)
        //.add_system(player_collide_platform)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let background: Handle<Image> = asset_server.load(BACKGROUND);
    let background_layer1: Handle<Image> = asset_server.load(BACKGROUND_LAYER1);
    let background_layer2: Handle<Image> = asset_server.load(BACKGROUND_LAYER2);
    let background_layer3: Handle<Image> = asset_server.load(BACKGROUND_LAYER3);

    // capture window size
    let window = windows.get_primary().unwrap();
    let (win_h, win_w) = (window.height(), window.width());
    let ground_lvl: f32 = -win_h / 2. + 67.;

    commands.insert_resource(WinSize { win_h, win_w });
    commands.spawn_bundle(SpriteBundle {
        texture: background_layer1,
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: background_layer2,
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            texture: background_layer3,
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            ground_level: ground_lvl,
            left_bound: -win_w / 2.,
            right_bound: win_w / 2.,
        });
}

/*          .insert(Platform {
    ground_level: ground_lvl + 50.,
    left_bound: 100. / 2.,
    right_bound: 150.,
})*/
/* let player_sprite = asset_server.load(CROUCH_SPRITE);
commands.spawn_bundle(SpriteBundle{
    texture : player_sprite,
    transform : Transform {
        translation : Vec3::new(0.,ground_lvl,1.),
        //scale : Vec3::new(2.,2.,1.),
        ..Default::default()
    },
    ..Default::default()
}); */

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn movement(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut Grounded,
            &mut Velocity,
            &mut Acceleration,
            &mut Transform,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    query_platforms: Query<&Platform>,
) {
    let delta = time.delta_seconds();
    for (mut grounded, mut velocity, mut acceleration, mut transform, texture_atlas) in
        query.iter_mut()
    {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
        velocity.vx += acceleration.ax * delta;
        velocity.vy += acceleration.ay * delta;
        let sprite_height = texture_atlases.get(texture_atlas).unwrap().size.y / 2.;
        //println!("velocity vy {}, acceleration ay{}, y_level : {}",velocity.vy,acceleration.ay, transform.translation.y);
        for platform in query_platforms.iter() {
            let ground_level = platform.ground_level;
            let left_bound = platform.left_bound;
            let right_bound = platform.right_bound;

            //println!("ground_level {ground_level}, left_bound {left_bound}, right_bound {right_bound}, y_level : {}", transform.translation.y);
            if grounded.0 == false {
                if ground_level + sprite_height + MARGIN >= transform.translation.y
                    && ground_level + sprite_height - MARGIN <= transform.translation.y
                    && transform.translation.x >= left_bound
                    && transform.translation.x <= right_bound
                {
                    acceleration.ay = 0.;
                    velocity.vy = 0.;
                    grounded.0 = true;
                } else {
                    acceleration.ay = -100.
                }
            }
        }
    }
}

/*
fn player_collide_platform(
    collision_: Res<Collision>,
    platform_query: Query<(&Transform, &SpriteSize), With<Platform>>,
    mut player_query: Query<(&mut Transform, &SpriteSize, &mut Velocity, &mut Grounded), With<Player>>,
) {
    // iterate through the player
    for (mut player_tf, player_size, mut velocity, mut grounded) in player_query.iter_mut() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        // iterate through the platforms
        for (platform_tf, platform_size) in platform_query.iter() {
            let platform_scale = Vec2::from(platform_tf.scale.xy());

            // determine if collision
            let collision = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                platform_tf.translation,
                platform_size.0 * platform_scale,
            );

            // perform collision logic
            if let Some(_) = collision {
                player_tf.translation.y = platform_tf.translation.y;
                grounded.0 = true;
            }
        }
    }
}
*/
