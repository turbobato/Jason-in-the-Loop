use bevy::prelude::*;

#[derive(Component)]
pub enum Direction{
    Up,
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);