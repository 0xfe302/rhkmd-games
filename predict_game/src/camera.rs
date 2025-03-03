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
    cam3d: Camera3d,
    cam: Camera,
    transform: Transform,
    tone: Tonemapping,
    bloom: Bloom,
}

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Startup, initialize_camera);
    app
        .insert_resource(AmbientLight::NONE)
        .insert_resource(ClearColor(Color::BLACK));
}

pub fn create_camera() -> FollowCameraBundle {
    //TODO can we attach to the thing here

    FollowCameraBundle{    
        tag: MainCamera,
        cam3d: Camera3d::default(),
        cam: Camera {
                 hdr: true,
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
        tone: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        bloom: Bloom::default(),           // 3. Enable bloom for the camera
    }
}