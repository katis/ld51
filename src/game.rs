use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationTag},
    GameAssets,
};

pub fn setup_game(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.player_atlas.clone(),
            transform: Transform::from_xyz(20., -40., 0.),
            ..default()
        })
        .insert(Animation::Start(assets.player_walk.clone()));
}
