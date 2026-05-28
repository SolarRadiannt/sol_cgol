use bevy::{
	prelude::*
};
use bevy_pancam::{ PanCam, PanCamPlugin, DirectionKeys};
use std::{collections::HashSet, time::Duration};

const TICK_RATE: f32 = 50./60.;
const PIXEL: f32 = 1.0;
const CELL_COLOR: Color = Color::srgb(1., 1., 1.);
const NEIGHBOR_OFFSETS: [IVec2; 8] = [
	IVec2::new(-1,  1),	IVec2::new(0,  1),	IVec2::new(1,  1),
	IVec2::new(-1,  0),								IVec2::new(1,  0),
	IVec2::new(-1, -1),	IVec2::new(0, -1),	IVec2::new(1, -1),
];

#[derive(Component)]
struct CompPosition(IVec2);

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Dead;

#[derive(Resource, Default)]
struct LiveCells(HashSet<IVec2>);

#[derive(Resource, Default)]
struct Paused(bool);

#[derive(Resource, Default)]
struct TickTimer(Timer);

fn spawn_cell(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: IVec2,
) -> Entity {
	let mesh = meshes.add(Cuboid::new(PIXEL, PIXEL, PIXEL));
	let material = materials.add(CELL_COLOR);
	
	commands.spawn((
		Cell,
		Mesh2d(mesh),
		MeshMaterial2d(material),
		CompPosition(position),
	)).id()
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		Camera2d,
		PanCam{
			grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
			move_keys: DirectionKeys {      // the keyboard buttons used to move the camera
				up:    vec![KeyCode::ArrowUp], // initalize the struct like this or use the provided methods for
				down:  vec![KeyCode::ArrowDown], // common key combinations
				left:  vec![KeyCode::ArrowLeft],
				right: vec![KeyCode::ArrowRight],
			},
    		..default()
		}
	));
}

fn get_cursor_position(
	window: Single<&Window>,
	camera: Single<(&Camera, &GlobalTransform)>,
) -> Vec2 {
	let window = window.into_inner();
	let (camera, camera_transform) = camera.into_inner();
	
	window
		.cursor_position()
		.and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
		.unwrap_or(Vec2::ZERO)
}

fn get_neighbors_count(pos: IVec2, live: &HashSet<IVec2>) -> u8 {
	NEIGHBOR_OFFSETS
		.iter()
		.filter(| offset | live.contains( &(pos + **offset) ))
		.count() as u8
}

fn condition_can_tick(
	paused: Res<Paused>,
	tick_timer: Res<TickTimer>,
) -> bool {	
	if !paused.0 {
		if tick_timer.0.just_finished() {
			return true
		}
 	};
	false
}

fn main() {
	let mut app = App::new();
	app.add_plugins((DefaultPlugins, PanCamPlugin));
	app.insert_resource(Paused(true));
	app.insert_resource(TickTimer(Timer::new(Duration::from_secs_f32(TICK_RATE), TimerMode::Repeating)));
	app.insert_resource(LiveCells::default());
	
	app.add_systems(Startup, (
		spawn_camera,
	));
	
	app.add_systems(Update, (
		system_tick_simulation,
		system_detect_pause,
		system_spawn_cell_on_hold,
		system_render,
	).chain());
	app.add_systems(FixedUpdate,
	(
		system_sync_live_cell,
		system_dead_marker,
		system_birth_cell,
		system_kill_cells,
		
	)
		.chain()
		.run_if(condition_can_tick)
	);
	
	app.run();
}

fn system_tick_simulation(
	mut tick_timer: ResMut<TickTimer>,
	time: Res<Time<Real>>,
) {
	tick_timer.0.tick(time.delta());
}

fn system_detect_pause(
	input: Res<ButtonInput<MouseButton>>,
	mut paused: ResMut<Paused>,
) {
	if input.just_pressed(MouseButton::Right) {
		paused.0 = !paused.0
	}
}

fn system_birth_cell(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	live: ResMut<LiveCells>,
) {
	let candidates: HashSet<IVec2> = live.0.iter()
		.flat_map(| pos | NEIGHBOR_OFFSETS.iter().map(|off| *pos + *off))
		.collect();
	
	for pos in candidates {
		if !live.0.contains(&pos) && get_neighbors_count(pos, &live.0) == 3 {
			spawn_cell(&mut commands, &mut meshes, &mut materials, pos);
		}
	}
}

fn system_sync_live_cell(
	mut live: ResMut<LiveCells>,
	query: Query<&CompPosition, (With<Cell>, Without<Dead>)>,
) {
	live.0.clear();
	for pos in query {
		live.0.insert(pos.0);
	};
}

fn system_dead_marker(
	mut commands: Commands,
	to_check: Query<(Entity, &CompPosition), (With<Cell>, Without<Dead>)>,
	live: Res<LiveCells>,
) {
	for (entity, pos) in to_check {
		let neighbors = get_neighbors_count(pos.0, &live.0);
		if neighbors < 2 || neighbors > 3 {
			commands.entity(entity).insert(Dead);
		};
	}
}

fn system_kill_cells(
	mut commands: Commands,
	to_kill: Query<Entity, With<Dead>>,
) {
	for entity in to_kill {
		commands.entity(entity).despawn();
	}
}


fn system_spawn_cell_on_hold(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	
	input: Res<ButtonInput<MouseButton>>,
	others: Query<&CompPosition, With<Cell>>,
	
	window: Single<&Window>,
	camera: Single<(&Camera, &GlobalTransform)>,
) {
	if !input.pressed(MouseButton::Left) { return; };
	
	let mouse_pos = get_cursor_position(window, camera)
    	.floor()
		.as_ivec2();
	
	println!("is holding");
	for position in others.iter() { if position.0 == mouse_pos { return }; };

	println!("spawning");
	spawn_cell(&mut commands, &mut meshes, &mut materials, mouse_pos);
}

fn system_render(
	mut to_render: Query<(&mut Transform, &CompPosition)>
) {
	for (mut transform, position) in &mut to_render {
		transform.translation = position.0
			.as_vec2()
			.extend(0.0);
	}
}
