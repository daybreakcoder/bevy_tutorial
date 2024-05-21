mod character;
mod hello_plugin;
mod simple_3d;

use bevy::prelude::*;
use hello_plugin::HelloPlugin;
use simple_3d::Simple3D;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin, Simple3D))
        .run();
}
