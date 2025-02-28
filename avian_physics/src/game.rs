#![allow(unused)]

use avian3d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButton;
use bevy::core_pipeline::{
    bloom::{Bloom, BloomCompositeMode},
    tonemapping::Tonemapping,
};
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};

use crate::utils::random_number;

use crate::camera::MainCamera;

// Component to identify Player
#[derive(Component)]
struct Player;

#[derive(Component)]
struct MovementTarget {
    pos: Transform,
}

// #[derive(Component)]
// struct MainCamera;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct GroundCoords {
    global: Vec3, // world-space coords
    local: Vec2, // relative to ground
}


// This is an example of the most simple plugin you can write, without
// having to implement any traits.
//
// If you wanted to toggle this plugin or configure it for the outside
// you can reach for a `PluginGroup`.

pub(super) fn plugin(app: &mut App) {
    // Your game logic here
    app
    // .insert_resource(AmbientLight::NONE)
    // .insert_resource(ClearColor(Color::BLACK))
    //.add_plugins(DefaultPlugins)
    // .add_plugins(DefaultPickingPlugins)
    .add_systems(Startup, setup)
    // .add_system(player1_movement)
    // .add_system(player2_movement)
    .add_systems(Update,player_movement)
    .add_systems(Update,mouse_click);
}

static WORLD_SIZE: f32 = 50.;

static CAM_DIST: f32 = 15.;

fn main() {
    App::new()

        // .add_systems(Update, camera_update)
        .run();
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

    // // pillars
    // for (x, z) in &[(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)] {
    //     commands.spawn((
    //         // RigidBody::Dynamic,
    //         // Collider::capsule(0.5, 1.5),
    //         Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
    //         MeshMaterial3d(stone.clone()),
    //         Transform::from_xyz(*x, 1.5, *z),
    //     ));
    // }

    // for n in 1..1000 {
    //     let val = n as f32;

    //     commands.spawn((
    //         // RigidBody::Kinematic,
    //         // Collider::sphere(0.5),
    //         Mesh3d(meshes.add(Sphere::new(0.5))),
    //         MeshMaterial3d(materials.add(Color::srgb(4.25, 7.4, 7.1))),
    //         Transform::from_xyz(-3.0 * val, 0., -7.0),
    //     ));
    //     commands.spawn((
    //         // RigidBody::Kinematic,
    //         // Collider::sphere(0.5),
    //         Mesh3d(meshes.add(Sphere::new(0.5))),
    //         MeshMaterial3d(materials.add(Color::srgb(4.25, 7.4, 7.1))),
    //         Transform::from_xyz(-3.0 * val, 0., 7.0),
    //     ));
    // }

 
    commands.spawn((
        Ground {},
        Mesh3d(meshes.add(Plane3d::default().mesh().size(WORLD_SIZE, WORLD_SIZE))),
        MeshMaterial3d(stone.clone()),
        // Visibility::Hidden,
        // RigidBody::Static,
        // Collider::cuboid(WORLD_SIZE, 0.1, WORLD_SIZE),
        PickingBehavior::IGNORE,
        //On::<Pointer<Move>>::run(change_hue_with_vertical_move),
    ));

    // // commands.spawn((
    // //     DirectionalLight::default(),
    // //     Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    // // ));

    // commands.spawn((
    //     // RigidBody::Dynamic,
    //     // Collider::sphere(0.5),
    //     Mesh3d(meshes.add(Sphere::new(0.5))),
    //     MeshMaterial3d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
    //     Transform::from_xyz(-10.0, 0.0, -10.0),
    //     LinearVelocity(Vec3{x: -1.0, y: 0., z: 0.}),
    // ));

    // Spawn player
    commands.spawn((
        Player {},
        MovementTarget{pos: Transform::from_translation(Vec3::ZERO)},
        // Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            //base_color: Color::srgb(6.25, 9.4, 9.1),
            //reflectance: 1.0,
            emissive: LinearRgba::rgb(13.99, 5.32, 2.0),
            // perceptual_roughness: 0.0,
            // metallic: 0.5,
            // alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        // RigidBody::Kinematic,
        // Collider::sphere(0.2),
        //Collider::capsule(0.5, 1.5),
        Transform::from_xyz(0.0, 10.0, 0.0),
        LinearVelocity(Vec3{x: -10.0, y: 0.0, z: 0.0}),
        //Mesh3d(meshes.add(Sphere::default())),
        // NotShadowCaster,
        // NotShadowReceiver,
    )).with_children(|parent|{
        // parent.spawn((    
        //     MainCamera,
        //     Camera3d::default(),
        //     Camera {
        //         hdr: true,
        //         ..default()
        //     },
        //     // DistanceFog {
        //     //     color: Color::srgb(0.0, 0.0, 0.0),
        //     //     falloff: FogFalloff::Linear {
        //     //         start: 5.0,
        //     //         end: 20.0,
        //     //     },
        //     //     ..default()
        //     // },
        //     //Transform::from_xyz(0.0, 0.5, 20.0).looking_at(Vec3::ZERO.with_y(0.5), Vec3::Y),
        //     Transform::from_xyz(20.0, 20.0, 0.0).looking_at(Vec3::new(-8., 1., 0.), Vec3::Y),
        //     Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        //     Bloom::default(),           // 3. Enable bloom for the camera
        // ));
        parent.spawn((
            // RigidBody::Kinematic,
            // Collider::sphere(0.5),
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(Color::srgb(4.25, 7.4, 7.1))),
            Transform::from_xyz(-3.0, 0., 0.0),
        ));
        parent.spawn((
            // RigidBody::Kinematic,
            // Collider::sphere(0.5),
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(Color::srgb(4.25, 7.4, 7.1))),
            Transform::from_xyz(3.0, 0., 0.0),
        ));

    //     // parent.spawn((
    //     //     PointLight {
    //     //         //color: Color::srgb(6.25, 9.4, 9.1),
    //     //         shadows_enabled: true,
    //     //         ..default()
    //     //     },
    //     //     //Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    //     // ));
    //     // parent.spawn((
    //     //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     //     MeshMaterial3d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
    //     //     // Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     //     // MeshMaterial3d(materials.add(StandardMaterial {
    //     //     //     base_color: Color::srgb(6.25, 9.4, 9.1),
    //     //     //     //reflectance: 1.0,
    //     //     //     emissive: LinearRgba::rgb(6.25, 90.4, 90.1),
    //     //     //     // perceptual_roughness: 0.0,
    //     //     //     // metallic: 0.5,
    //     //     //     // alpha_mode: AlphaMode::Blend,
    //     //     //     ..default()
    //     //     // })),
    //     // ));
    });
}

// pub fn camera_update(
//     q_cam: Query<&mut Transform, With<MainCamera>>,
//     q_player: Query<&GlobalTransform, With<Player>>, 
// ) {
//     let player = q_player.single();
//     let cam = q_cam.single();

//     // cam.translation.x = player.translation.x + CAM_DIST;
//     // cam.translation.z = player.translation.z;
//     // cam.translation.y = player.translation.y + CAM_DIST;
// }

fn mouse_click(
    // mut coords: ResMut<GroundCoords>,
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_ground: Query<&GlobalTransform, With<Ground>>,
    touches: Res<Touches>,
    mut q_move_target: Query<&mut MovementTarget, With<Player>>,
    mut gizmos: Gizmos,
)
{
    // TODO look at bevy_mod_picking
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
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
        gizmos.circle(
            Isometry3d::new(
                global_cursor + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.5,
            Color::srgb(9.25, 6.4, 6.1),
        );
    }
}

fn player_movement(
    mut q_pos: Query<&mut Transform, With<Player>>,
    q_tar: Query<&MovementTarget, With<Player>>,
    mut q_cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
)
{
    let mut pos = q_pos.single_mut();
    let tar = q_tar.single();
    let mut cam = q_cam.single_mut();
    // let mut direction = pos.looking_at(tar.pos.translation, Vec3::Y).forward();

    pos.translation = pos.translation.move_towards(tar.pos.translation, 0.4);

    // if pos.translation.distance(tar.pos.translation) > 1.0 {
    //     pos.translation += direction * 1.0;
    //     // cam.translation = pos.translation.move_towards(rhs, d)
    //     // cam.look_to(direction, Vec3::Y);
    // }
}
