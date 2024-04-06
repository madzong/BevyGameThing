use bevy::prelude::*;
use plugin::plugin::HelloPlugin;

mod plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}