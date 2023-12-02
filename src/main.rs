use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use player::PlayerPlugin;
use components::{
	Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable,
	Player, SpriteSize, Velocity,
};

mod components;
mod player;

// region:    --- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

// region:    --- Game Constants
const BASE_SPEED: f32 = 500.;
const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 2;
const FORMATION_MEMBERS_MAX: u32 = 2;

#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
	player: Handle<Image>,
	player_laser: Handle<Image>,
	enemy: Handle<Image>,
	enemy_laser: Handle<Image>,
	explosion: Handle<TextureAtlas>,
}

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct PlayerState {
	on: bool,       // alive
	last_shot: f64, // -1 if not shot
}
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: -1.,
		}
	}
}

impl PlayerState {
	pub fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	pub fn spawned(&mut self) {
		self.on = true;
		self.last_shot = -1.;
	}
}


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
		.add_systems(Startup, setup_system)
		.add_systems(Update, movable_system)
		.run();
}


fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	query: Query<&Window, With<PrimaryWindow>>,
) {
	// camera
	commands.spawn(Camera2dBundle::default());

	// capture window size
	let Ok(primary) = query.get_single() else {
		return;
	};
	let (win_w, win_h) = (primary.width(), primary.height());

	// position window (for tutorial)
	// window.set_position(IVec2::new(2780, 4900));

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	// create explosion texture atlas
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	let texture_atlas =
		TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
	let explosion = texture_atlases.add(texture_atlas);

	// add GameTextures resource
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		player_laser: asset_server.load(PLAYER_LASER_SPRITE),
		enemy: asset_server.load(ENEMY_SPRITE),
		enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
		explosion,
	};
	commands.insert_resource(game_textures);
	commands.insert_resource(EnemyCount(0));
}


fn movable_system(
	mut commands: Commands,
	time: Res<Time>,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
	let delta = time.delta_seconds();

	for (entity, velocity, mut transform, movable) in &mut query {
		let translation = &mut transform.translation;
		translation.x += velocity.x * delta * BASE_SPEED;
		translation.y += velocity.y * delta * BASE_SPEED;

		if movable.auto_despawn {
			// despawn when out of screen
			const MARGIN: f32 = 200.;
			if translation.y > win_size.h / 2. + MARGIN
				|| translation.y < -win_size.h / 2. - MARGIN
				|| translation.x > win_size.w / 2. + MARGIN
				|| translation.x < -win_size.w / 2. - MARGIN
			{
				commands.entity(entity).despawn();
			}
		}
	}
}