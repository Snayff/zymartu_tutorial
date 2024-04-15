use bevy::app::{App, Plugin, Startup};
use bevy::input::keyboard::KeyboardInput;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Velocity, MovingObjectBundle, Acceleration};


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;


#[derive(Component, Debug)]
pub struct Spaceship;
#[derive(Component, Debug)]
pub struct SpaceshipMissile;
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostStartup, spawn_spaceship)
			.add_systems(Update,
	    (spaceship_movement_controls, spaceship_weapon_controls,)
			);
	}
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
	commands.spawn(
		(
			MovingObjectBundle {
				velocity: Velocity::new(Vec3::ZERO),
				acceleration: Acceleration::new(Vec3::ZERO),
				collider: Collider::new(SPACESHIP_RADIUS),
				model: SceneBundle {
					scene: scene_assets.spaceship.clone(),
					transform: Transform::from_translation(STARTING_TRANSLATION),
					..default()
				},
			},
				Spaceship,
		)
	);
}

fn spaceship_movement_controls(
	mut query: Query<(&mut Transform, &mut Velocity), With <Spaceship>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let (mut transform, mut velocity) = query.single_mut();
	let mut rotation = 0.0;
	let mut roll = 0.0;
	let mut movement = 0.0;

	// rotate
	if keyboard_input.pressed(KeyCode::KeyD) {
		rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
	} else if keyboard_input.pressed(KeyCode::KeyA) {
		rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
	}

	// move forward or back - N.B. mult by delta time elsewhere
	if keyboard_input.pressed(KeyCode::KeyS) {
		movement = -SPACESHIP_SPEED;
	} else if keyboard_input.pressed(KeyCode::KeyW) {
		movement = SPACESHIP_SPEED;
	}

	// roll
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
	} else if keyboard_input.pressed(KeyCode::ControlLeft) {
		roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
	}


	// rotate around the Y axis
	// ignore the z axis rotation. forces remaining on 0 z axis.
	transform.rotate_y(rotation);

	// rotate around the local z axis
	// rotation is relative to current rotation
	transform.rotate_local_z(roll);

	// move forward in faced direction
	// negative to reflect diff between model and bevy's expectations
	velocity.value = -transform.forward() * movement;

}

fn spaceship_weapon_controls(
	mut commands: Commands,
	query: Query<&Transform, With<Spaceship>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	scene_assets: Res<SceneAssets>,
) {
	let transform = query.single();

	if keyboard_input.pressed(KeyCode::Space) {
		commands.spawn(
			(
				MovingObjectBundle {
					velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
					acceleration: Acceleration::new(Vec3::ZERO),
					collider: Collider::new(MISSILE_RADIUS),
					model: SceneBundle {
						scene: scene_assets.missiles.clone(),
						transform: Transform::from_translation(
							transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR
						),
						..default()
					},
				},
				SpaceshipMissile,
				)
		);
	}
}