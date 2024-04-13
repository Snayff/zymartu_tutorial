mod spaceship;
mod movement;
mod debug;
mod camera;
mod asteroids;

use bevy::prelude::*;
use crate::asteroids::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;


fn main() {
	App::new()
		// built ins
		.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 750.0,
		})
		.add_plugins(DefaultPlugins)
		
		// custom plugins
		.add_plugins(SpaceshipPlugin)
		.add_plugins(AsteroidPlugin)
		.add_plugins(CameraPlugin)
		.add_plugins(MovementPlugin)
		.add_plugins(DebugPlugin)
		
		.run()
}





