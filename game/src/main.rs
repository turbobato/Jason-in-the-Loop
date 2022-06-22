mod components;
mod player;

use bevy::{prelude::*, render::texture::ImageSettings, math::const_vec3};

const PLAYER_SPEED : f32 = 200.;
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SIZE: Vec3 = const_vec3!([120.0, 80.0, 0.0]);
struct GroundLevel(f32);

struct WinSize{
    win_h : f32,
    win_w : f32
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
        title: "ProjetX".to_string(),
        width: 928.0,
        height: 793.0,
        resizable: false,
        ..Default::default()
    })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let (win_h,win_w) = (793.,928.); 
    let ground_lvl :f32 = -win_h/2. + 103.;
    let texture_handle = asset_server.load("textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(120.0, 80.0), 10, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());

    // Animation player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {translation: Vec3::new(0.,ground_lvl,1.),..default()},
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    
    // Forest background
    commands
        .spawn_bundle(SpriteBundle{
        texture: asset_server.load("textures/forest/Free Pixel Art Forest/Preview/Background.png"),
        transform: Transform {translation: Vec3::new(0.,0.,0.),..default()},
        ..default()
    });

}

#[derive(Component)]
struct Player;
    
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
        }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
        }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position = player_transform.translation.x + direction * PLAYER_SPEED * TIME_STEP;


    }