
#![allow(unused)]
#![allow(deadcode)]

use bevy::window::{WindowResolution, PresentMode};
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButton;
use bevy::core_pipeline::{
    bloom::{Bloom, BloomCompositeMode},
    tonemapping::Tonemapping,
};
use bevy::prelude::*;
use crate::camera::{MainCamera, create_camera};
use bevy::ecs::entity::Entities;

static WORLD_SIZE: f32 = 50.;

static CAM_DIST: f32 = 15.;

// Component to identify Player
#[derive(Component)]
struct Player;

#[derive(Component)]
struct MovementTarget {
    pos: Transform,
}

#[derive(Component)]
struct Prediction;

#[derive(Component)]
struct Countdown(Timer);

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct ScoreLine;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    InGame,
    MovingEndCam,
    DrawingScoreLine,
    End,
} 

// fn prediction_made() -> impl Condition<()> {
//     Entities::contains(&self,Countdown)
//     Entities::contains(&self, entity)
// }

pub(super) fn plugin(app: &mut App) {
    // Your game logic here
    app
    .init_state::<AppState>()
    .add_systems(Startup, setup)
    .add_systems(Update,player_movement.run_if(in_state(AppState::InGame)))
    .add_systems(Update,mouse_click.run_if(not(any_with_component::<Prediction>)))
    .add_systems(FixedUpdate,(tick_countdown, detect_stop).run_if(any_with_component::<Prediction>.and(in_state(AppState::InGame))))
    .add_systems(Update, move_camera_to_prediction.run_if(in_state(AppState::MovingEndCam)))
    .add_systems(Update, draw_score_line.run_if(in_state(AppState::DrawingScoreLine)));
    //.add_systems(Update, calc_score.run_if(in_state(AppState::End)))
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0., 0., 0.), //Srgba::hex("28221B").unwrap().into(),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Ground {},
        Mesh3d(meshes.add(Plane3d::default().mesh().size(WORLD_SIZE, WORLD_SIZE))),
        MeshMaterial3d(stone.clone()),
        Visibility::Hidden,
    ));

    // Spawn player
    commands.spawn((
        Player {},
        MovementTarget{pos: Transform::from_translation(Vec3::ZERO)},
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            ..default()
        })),
    )).with_children(|parent|{
        parent.spawn(create_camera());
        // parent.spawn((
        //     PointLight {
        //         shadows_enabled: true,
        //         ..default()
        //     },
        //     //Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        // ));
    });
}

pub fn mouse_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut coords: ResMut<GroundCoords>,
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform, Entity), With<MainCamera>>,
    q_ground: Query<&GlobalTransform, With<Ground>>,
    touches: Res<Touches>,
    mut q_move_target: Query<&mut MovementTarget, With<Player>>,
    mut gizmos: Gizmos,
)
{
    // TODO look at bevy_mod_picking
    let window = q_window.single();
    let (camera, camera_transform, camera_entity) = q_camera.single();
    let ground = q_ground.single();
    let mut move_target = q_move_target.single_mut();

    let mut direction = Vec3::ZERO;

    if mouse.pressed(MouseButton::Left) || touches.any_just_pressed() {
        let Some(cursor_position) = touches.first_pressed_position()
            .or_else(|| window.cursor_position()) else {
                return;
        };
    
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
    
        // Calculate if and where the ray is hitting the ground plane.
        let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
        else {
            return;
        };
        let global_cursor = ray.get_point(distance);
        let inverse_transform_matrix = ground.compute_matrix().inverse();
        let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);
        //println!("World coords: {}/{}/{}", point.x, point.y, point.z);
        move_target.pos.translation.x = local_cursor.x;
        move_target.pos.translation.z = local_cursor.z;
        commands.spawn((
            Prediction,
            //Mesh3d(meshes.add(Circle::new(0.5))),
            MeshMaterial3d(materials.add(Color::srgb(9.25, 6.4, 6.1))), 
            Mesh3d(meshes.add(Sphere::new(0.2))),
            //Transform::from_translation(global_cursor + ground.up() * 0.01).rotate(Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3())),
            Transform::from_translation(move_target.pos.translation),
        ));

        commands.spawn(Countdown(Timer::from_seconds(1.0, TimerMode::Once)));
        commands.entity(camera_entity).remove_parent_in_place();
    }
}

pub fn player_movement(
    mut q_pos: Query<&mut Transform, With<Player>>,
    q_tar: Query<&MovementTarget, With<Player>>,
    mut q_cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    // time: Res<Time>,
)
{
    let mut pos = q_pos.single_mut();
    let tar = q_tar.single();
    let mut cam = q_cam.single_mut();
    //pos.translation = pos.translation.move_towards(tar.pos.translation, 0.5);
    //TODO this will follow the "pattern"
    pos.translation.x -= 0.5; //= pos.translation.move_towards(tar.pos.translation, 0.5);
    //cam.translation.x = pos.translation.x - 5.0;
    //cam.translation.y = pos.translation.y - 10.0;

}

pub fn tick_countdown(
    mut q_timer: Query<&mut Countdown>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
)
{
    let mut timer = q_timer.single_mut();

    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(AppState::MovingEndCam);
    }

}

fn detect_stop(
    q_player: Query<&Transform, With<Player>>,
    q_prediction: Query<&Transform, (With<Prediction>, Without<Player>)>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
)
{
    let player = q_player.single();
    let prediction = q_prediction.single();

    if player.translation.x <= prediction.translation.x {
        // STOP
        // HWO
        // STATE?
        // next_state.set(AppState::MovingEndCam);

    }    

}

fn move_camera_to_prediction(
    q_prediction: Query<&Transform, (With<Prediction>, Without<MainCamera>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>, Without<Prediction>)>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Prediction>, Without<Player>)>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
)
{

    
    let prediction = q_prediction.single();
    let player = q_player.single();

    let Ok(mut cam) = q_camera.get_single_mut() else {
        return;
    };
    //TODO configure height and how far back
    let mut midpoint = prediction.translation.midpoint(player.translation).with_y(15.);
    midpoint.x += 20.;

    let prev_pos = cam.translation;
    cam.translation = cam.translation.move_towards(midpoint, 0.3);

    if cam.translation == prev_posh {

        next_state.set(AppState::DrawingScoreLine);
    } 
}

fn draw_score_line(

    q_prediction: Query<&Transform, (With<Prediction>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<Prediction>)>,
)
{

}