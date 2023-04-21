use bevy::prelude::*;

use crate::components::Position;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);
