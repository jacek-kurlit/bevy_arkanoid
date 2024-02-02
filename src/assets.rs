use bevy::prelude::*;

pub struct AssetsPlugin;

#[derive(Resource, Default)]
pub struct LoadedAssets {
    pub paddle: Handle<Image>,
    pub red_brick: Handle<Image>,
    pub blue_brick: Handle<Image>,
    pub pink_brick: Handle<Image>,
    pub yellow_brick: Handle<Image>,
    pub ball: Handle<Image>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut loaded_assets: ResMut<LoadedAssets>) {
    *loaded_assets = LoadedAssets {
        paddle: asset_server.load("paddle.png"),
        yellow_brick: asset_server.load("yellow_brick.png"),
        red_brick: asset_server.load("red_brick.png"),
        blue_brick: asset_server.load("blue_brick.png"),
        pink_brick: asset_server.load("pink_brick.png"),
        ball: asset_server.load("ball.png"),
    };
}
