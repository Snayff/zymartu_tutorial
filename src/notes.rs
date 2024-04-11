///////////// RESOURCES //////////////////

// If there is only one global instance (singleton) of something, and it is standalone
// (not associated with other data), create a Resource.

// Example:
// #[derive(Resource)]
// struct GameSettings {
//     current_level: u32,
//     difficulty: u32,
//     max_time_seconds: u32,
// }
//
// fn setup_game(mut commands: Commands) {
//     // Add the GameSettings resource to the ECS
//     // (if one already exists, it will be overwritten)
//     commands.insert_resource(GameSettings {
//         current_level: 1,
//         difficulty: 100,
//         max_time_seconds: 60,
//     });
// }

// example use of calling resource:
// fn spawn_extra_enemies(
//     mut commands: Commands,
//     // we can easily access our resource from any system
//     game_settings: Res<GameSettings>,
// ) {
//     if game_settings.difficulty > 50 {
//         commands.spawn((
//             // ...
//             ));
//     }
// }

///////// COMPONENTS  /////////////

// Data stored using Entities and Components is accessed using queries.
// For example, if you want to implement a new game mechanic, write a system
// (just a Rust function that takes special parameters), specify what component types you want to
// access, and do your thing. You can either iterate through all entities that match your query,
// or access the data of a specific one (using the Entity ID).

// Example:
// #[derive(Component)]
// struct Xp(u32);
//
// #[derive(Component)]
// struct Health {
//     current: u32,
//     max: u32,
// }
//
// fn level_up(
//     // We want to access the Xp and Health data:
//     mut query: Query<(&mut Xp, &mut Health)>,
// ) {
//     // process all relevant entities
//     for (mut xp, mut health) in query.iter_mut() {
//         if xp.0 > 1000 {
//             xp.0 -= 1000;
//             health.max += 25;
//             health.current = health.max;
//         }
//     }
//}

///////// SYSTEMS //////////////

// These functions can only take special parameter types, to specify what data you need access to.
// If you use unsupported parameter types in your function, you will get confusing compiler errors!
//
// Some of the possibilities are:
//
//     accessing resources using Res/ResMut
//     accessing components of entities using queries (Query)
//     creating/destroying entities, components, and resources using Commands (Commands)
//     sending/receiving events using EventWriter/EventReader

// #### Non Exclusive Systems #####

// Example:
// (Note how, just by looking at the function parameters, we know exactly what data can be accessed.)
// fn enemy_detect_player(
//     // access data from resources
//     mut ai_settings: ResMut<EnemyAiSettings>,
//     gamemode: Res<GameModeData>,
//     // access data from entities/components
//     query_player: Query<&Transform, With<Player>>,
//     query_enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
//     // in case we want to spawn/despawn entities, etc.
//     mut commands: Commands,
// ) {
//     // ... implement your behavior here ...
// }

//  #### Exclusive Systems ####

// Exclusive systems provide you with a way to get full direct access to the ECS World.
// They cannot run in parallel with other systems, because they can access anything and do anything.
// Sometimes, you might need this additonal power.

// Example:
// fn save_game(
//     // get full access to the World, so we can access all data and do anything
//     world: &mut World,
// ) {
//     // ... save game data to disk, or something ...
// }

////////////// SCHEDULES /////////////

// Bevy stores systems inside of schedules (Schedule). The schedule contains the systems and all
// relevant metadata to organize them, telling Bevy when and how to run them. Bevy Apps typically #
// contain many schedules. Each one is a collection of systems to be invoked in different
// scenarios (every frame update, fixed timestep update, at app startup,
// on state transitions, etc.).

// Within schedules, systems can be grouped into sets. Sets allow multiple systems to share
// common configuration/metadata. Systems inherit configuration from all sets they belong to.
// Sets can also inherit configuration from other sets.

// Example:
// // Set configuration is per-schedule. Here we do it for `Update`
// app.configure_sets(Update, (
//     MainMenuSet
//         .run_if(in_state(MainMenu)),
//     GameplaySet
//         .run_if(in_state(InGame)),
//     InputSet
//         .in_set(GameplaySet),
//     EnemyAiSet
//         .in_set(GameplaySet)
//         .run_if(not(cutscene))
//         .after(player_movement),
//     AudioSet
//         .run_if(not(audio_muted)),
// ));
// app.add_systems(Update, (
//     (
//         ui_button_animate,
//         menu_logo_animate.in_set(MainMenuSet),
//     ),
//     (
//         enemy_movement,
//         enemy_spawn,
//         enemy_despawn.before(enemy_spawn),
//     ).in_set(EnemyAiSet),
//     (
//         mouse_input.run_if(mouse_enabled),
//         controller_input.run_if(gamepad_enabled),
//     ).in_set(InputSet),
//     (
//         footstep_sound.in_set(GameplaySet),
//         menu_button_sound.in_set(MainMenuSet),
//         background_music,
//     ).in_set(AudioSet),
//     (
//         player_movement
//             .run_if(player_alive)
//             .run_if(not(cutscene)),
//         camera_movement,
//     ).in_set(GameplaySet).after(InputSet),
// ));
