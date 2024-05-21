mod character;
mod simple_3d;

use bevy::prelude::*;
use simple_3d::Simple3D;

fn main() {
    App::new().add_plugins((DefaultPlugins, Simple3D)).run();
}
