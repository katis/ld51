mod animation;
mod game;
mod sheet_loader;
mod state;

use animation::{animation_system, Animation, AnimationTag};
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use game::setup_game;
use sheet_loader::AtlasLoaderPlugin;
use state::GameState;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "sheets/enemies.sheet.json")]
    pub enemy_atlas: Handle<TextureAtlas>,
    #[asset(path = "sheets/enemies.sheet.json#Walk")]
    pub enemy_walk: Handle<AnimationTag>,
    #[asset(path = "sheets/player.sheet.json")]
    pub player_atlas: Handle<TextureAtlas>,
    #[asset(path = "sheets/player.sheet.json#Walk")]
    pub player_walk: Handle<AnimationTag>,
}

fn main() {
    App::new()
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::Playing)
                .with_collection::<GameAssets>(),
        )
        .add_state(GameState::LoadingAssets)
        .insert_resource(WindowDescriptor {
            title: "Slower defense".to_string(),
            width: 1024.,
            height: 1024.,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(AtlasLoaderPlugin)
        .add_plugin(LdtkPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup)
                .with_system(setup_game),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(init_enemies)
                .with_system(animation_system),
        )
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            set_clear_color: SetClearColor::FromEditorBackground,
            ..default()
        })
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.;
    camera.projection.scaling_mode = ScalingMode::Auto {
        min_width: 256.,
        min_height: 256.,
    };
    camera.transform.translation.x = 128.;
    camera.transform.translation.y = 128.;
    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/map1.ldtk"),
        ..Default::default()
    });
}

fn init_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    assets: Res<GameAssets>,
) {
    for (enemy, transform, instance) in query.iter() {
        if let "BasicEnemy" = instance.identifier.as_str() {
            commands
                .entity(enemy)
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: assets.enemy_atlas.clone(),
                    transform: *transform,
                    ..default()
                })
                .insert(Enemy)
                .insert(Animation::Start(assets.enemy_walk.clone()));
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct Enemy;
