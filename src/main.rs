use assets::AssetsPlugin;
use ball::BallPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use bricks::BrickPlugin;
use paddle::PaddlePlugin;
use walls::WallPlugin;

mod assets;
mod ball;
mod bricks;
mod paddle;
mod walls;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("3F7CB6").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        // custom
        .add_plugins(AssetsPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(WallPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    //TODO: check units from tutorial, mayb they can be unified?
    commands.spawn(Camera2dBundle::default());
}
