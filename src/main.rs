

fn main() {
    App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Rust Invaders!".into(),
				resolution: (598., 676.).into(),
				..Default::default()
			}),
			..Default::default()
		}))
		.add_plugins(PlayerPlugin)
		.add_plugins(EnemyPlugin)
		.add_systems(Startup, setup_system)
		.add_systems(Update, movable_system)
		.add_systems(Update, player_laser_hit_enemy_system)
		.add_systems(Update, enemy_laser_hit_player_system)
		.add_systems(Update, explosion_to_spawn_system)
		.add_systems(Update, explosion_animation_system)
		.run();
}
