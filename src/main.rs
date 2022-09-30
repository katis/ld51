mod animation;
mod game;
mod sheet_loader;

use animation::animation_system;
use bevy::prelude::*;
use game::setup_game;
use sheet_loader::AtlasLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AtlasLoaderPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_game)
        .add_system(bevy::window::close_on_esc)
        .add_system(animation_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
