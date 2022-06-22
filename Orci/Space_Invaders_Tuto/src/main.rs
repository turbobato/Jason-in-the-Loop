use bevy::prelude::*;
//region: ---ASSET CONSTANT

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32,f32) = (144., 75.);

// endregion: ---ASSET CONSTANT
fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.,0.,0.5)))
		.insert_resource(WindowDescriptor{
			title: "Projet X".to_string(),
			width: 600.0,
			height: 700.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup_system)
		.run()
	}

	fn setup_system(mut commands: Commands, 
		asset_serveur: Res<AssetServer>, 
		mut windows: ResMut<Windows>,
	) {
		//camera
		commands.spawn_bundle(OrthographicCameraBundle::new_2d());

		//window size 
		let window = windows.get_primary_mut().unwrap();
		let (win_w, win_h) = (window.width(), window.height());
		
		//position of window
		window.set_position(IVec2::new(2780, 4900));

		//player
		commands.spawn_bundle(SpriteBundle{
			texture: asset_serveur.load(PLAYER_SPRITE),
			..Default::default()
		});
	}