use bevy::prelude::*;
use bevy::core_pipeline::{
    bloom::{Bloom, BloomCompositeMode},
    tonemapping::Tonemapping,
};

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

#[derive(Bundle)]
pub struct FollowCameraBundle {
    tag: MainCamera,
    cam: Camera3dBundle,
}

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Startup, initialize_camera);
}

pub fn create_camera() -> FollowCameraBundle {
    //TODO can we attach to the thing here

    FollowCameraBundle{    
        tag: MainCamera,
        cam: Camera3dBundle {
            camera: Camera {
                // =hdr: true,
                ..default()
            },
            // DistanceFog {
            //     color: Color::srgb(0.0, 0.0, 0.0),
            //     falloff: FogFalloff::Linear {
            //         start: 5.0,
            //         end: 20.0,
            //     },
            //     ..default()
            // },
            //Transform::from_xyz(0.0, 0.5, 20.0).looking_at(Vec3::ZERO.with_y(0.5), Vec3::Y),
            transform: Transform::from_xyz(20.0, 20.0, 0.0).looking_at(Vec3::new(-8., 1., 0.), Vec3::Y),
            // Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            // Bloom::default(),           // 3. Enable bloom for the camera
            ..default()
        },
    }
}