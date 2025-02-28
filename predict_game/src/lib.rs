use bevy::prelude::*;

mod camera;
mod debug;
mod game;
mod input;
mod utils;
mod window;
//mod physics;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::plugin,
            camera::plugin,
            input::plugin,
            game::plugin,
            //physics::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((
            //dev_tools::plugin,
            debug::plugin
        ));
    }
}
