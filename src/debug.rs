use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, print_position);
	}
}

fn print_position(query: Query<(Entity, &Transform)>) {
	// log the position of every entity
	for (entity, transform) in query.iter() {
		info!("Entity {:?} is at position {:?}, ", entity, transform);
	}
}