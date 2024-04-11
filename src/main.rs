mod spaceship;
mod movement;
mod debug;
mod camera;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;


fn main() {
	App::new()
		// built ins
		.add_plugins(DefaultPlugins)
		
		// custom plugins
		.add_plugins(SpaceshipPlugin)
		.add_plugins(CameraPlugin)
		.add_plugins(MovementPlugin)
		.add_plugins(DebugPlugin)
		
		.run()
}





