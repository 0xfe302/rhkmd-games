#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

use predict::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}