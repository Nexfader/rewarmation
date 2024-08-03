use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use rewarmation::{
    game::{GamePlayState, GamePlugin, PlayingState},
    input::InputPlugin,
    physics::PhysicsPlugin,
    player::{spawn_player, PlayerPlugin},
    terrain::{spawn_terrain, TerrainPlugin},
    weapon::{attach_bazooka, WeaponPlugin},
};

fn main() {
    const SKY_COLOR: Color = Color::srgb(0.529, 0.808, 0.922);

    App::new()
        .add_plugins((
            DefaultPlugins,
            PanCamPlugin,
            InputPlugin,
            TerrainPlugin,
            GamePlugin,
            PhysicsPlugin,
            PlayerPlugin,
            WeaponPlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(SKY_COLOR))
        .insert_state(GamePlayState::InGame)
        .insert_state(PlayingState::Playing)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        PanCam {
            max_scale: Some(2.0),
            min_y: Some(0.0),
            ..default()
        },
    ));

    spawn_terrain(&mut commands, asset_server.load("textures/map.png"));

    let player = spawn_player(&mut commands, &asset_server, Vec2::new(0.0, 1000.0));
    attach_bazooka(&mut commands, &asset_server, player);
}
