use bevy::{
	ecs::system::SystemParam, prelude::*, sprite_render::Material2d
};
use bevy_pancam::{ PanCam, PanCamPlugin, DirectionKeys};
use web_sys::js_sys::Number;

const PIXEL: f32 = 1.0;
const CELL_COLOR: Color = Color::srgb(1., 1., 1.);

#[derive(Component)]
struct CompMoveDirection(Vec2);

#[derive(Component)]
struct CompPosition(Vec2);

#[derive(Component)]
#[require(
	CompMoveDirection(Vec2::ZERO)
)]
struct Cell;

#[derive(Component)]
struct NeighborCount(u8);

#[derive(SystemParam)]
struct ParamsSpawnCell <'w, 's>{
	commands: Commands<'w, 's>,
	meshes: ResMut<'w, Assets<Mesh>>,
	materials: ResMut<'w, Assets<ColorMaterial>>,
}

fn spawn_cell(
	mut params: ParamsSpawnCell,
	position: Vec2,
) -> Entity {
	let mut cmds = params.commands;
	let mesh = params.meshes
		.add(Cuboid::new(PIXEL, PIXEL, PIXEL));
	let material = params.materials.add(CELL_COLOR);
	
	cmds.spawn((
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
	window: Single<(&Window)>,
	camera: Single<(&Camera, &GlobalTransform)>,
) -> Vec2 {
	let window = window.into_inner();
	let (camera, camera_transform) = camera.into_inner();
	
	window
		.cursor_position()
		.and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
		.unwrap_or(Vec2::ZERO)
}

fn main() {
	let mut app = App::new();
	app.add_plugins((DefaultPlugins, PanCamPlugin));
	app.add_systems(Startup, (
		spawn_camera,
	));
	app.add_systems(FixedUpdate, (
		system_render,
		system_move,
		system_spawn_cell_on_hold,
	).chain());
	
	app.run();
}

fn system_spawn_cell_on_hold(
	params: ParamsSpawnCell,
	input: Res<ButtonInput<MouseButton>>,
	others: Query<&CompPosition, With<Cell>>,

	window: Single<&Window>,
	camera: Single<(&Camera, &GlobalTransform)>,
) {
	if !input.pressed(MouseButton::Left) { return; };
	
	let mouse_pos = get_cursor_position(window, camera)
		.floor();
	
	println!("is holding");
	for position in others.iter() { if position.0 == mouse_pos { return }; };

	println!("spawning");
	spawn_cell(params, mouse_pos);
}


fn system_move(
	to_move: Query<(&mut CompPosition, &CompMoveDirection)>
) {
	for (mut position, move_dir) in to_move {
		position.0 += move_dir.0
	}
}

fn system_render(
	mut to_render: Query<(&mut Transform, &CompPosition)>
) {
	for (mut transform, position) in &mut to_render {
		transform.translation = position.0.extend(0.0);
	}
}
