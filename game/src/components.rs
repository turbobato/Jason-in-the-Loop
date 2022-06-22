use bevy::prelude::*;


// region : general components

#[derive(Component)]
pub enum Movement{
    Idle,
    Running,
}

#[derive(Component, Default)]
pub struct Velocity {
	pub vx : f32,
	pub vy : f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
//endregion : general components 

//region : player components

#[derive(Component)]
pub struct Player;

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct Enemy;

//endregion : enemy components 