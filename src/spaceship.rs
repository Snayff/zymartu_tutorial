use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::movement::{Velocity, MovingObjectBundle, Acceleration};


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_spaceship);
	}
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(
		MovingObjectBundle {
			velocity: Velocity {
				value: STARTING_VELOCITY,
			},
			acceleration: Acceleration::new(Vec3::ZERO),
			model: SceneBundle {
				scene: asset_server.load("Spaceship.glb#Scene0"),
				transform: Transform::from_translation(STARTING_TRANSLATION),
				..default()
			},
		}
	);
}
