


use bevy::prelude::*;
use bevy::window::{WindowResolution, PresentMode};

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

pub(super) fn plugin(app: &mut App) {
    // let primary_window = Window {
    //     title: "Bevy game".into(),
    //     resizable: false,
    //     resolution: WindowResolution::new(700., 1400.).with_scale_factor_override(1.0),
    //     canvas: Some("#bevy".to_owned()),
    //     desired_maximum_frame_latency: core::num::NonZero::new(1u32),
    //     present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
    //     fit_canvas_to_parent: true,
    //     ..default()
    // };

    let primary_window = Window {
        title: "Bevy game".into(),
        resizable: false,
        resolution: WindowResolution::new(700., 1400.).with_scale_factor_override(1.0),
        //canvas: Some("#bevy".to_owned()),
        //desired_maximum_frame_latency: core::num::NonZero::new(1u32),
        //present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
        fit_canvas_to_parent: true,
        ..default()
    };

    app
        .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(primary_window),
                ..default()
        }));
}
