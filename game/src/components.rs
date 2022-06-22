use bevy::prelude::*;


// region : general components

#[derive(Component)]
pub enum Movement{
    Idle,
    Running,
}

#[derive(Component)]
pub enum Direction {
	
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
//endregion : general components 

//region : player components

#[derive(Component)]
pub struct PlayerComponent;

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct EnemyComponent;

//endregion : enemy components 