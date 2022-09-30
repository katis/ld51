use bevy::prelude::*;

use crate::animation::{Animation, AnimationTag};

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_atlas_handle: Handle<TextureAtlas> = asset_server.load("sheets/player.sheet.json");
    let walk_anim_handle: Handle<AnimationTag> = asset_server.load("sheets/player.sheet.json#Walk");

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            transform: Transform::from_xyz(20., 20., 0.),
            ..default()
        })
        .insert(Animation::Start(walk_anim_handle));
}
