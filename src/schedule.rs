use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
	UserInput, 
	EntityUpdates, 
	CollisionDetection, 
	DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
	fn build(&self, app: &mut App) {
		app.configure_sets(
			Update,
			(
				InGameSet::DespawnEntities,
				// flush commands
				InGameSet::UserInput,
				InGameSet::EntityUpdates,
				InGameSet::CollisionDetection,
				)
				.chain(),
		)
		.add_systems(
			Update,
			// flush commands after despawn but before user input
			apply_deferred
				.after(InGameSet::DespawnEntities)
				.before(InGameSet::UserInput),
		);
	}
}