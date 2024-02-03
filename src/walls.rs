use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, RigidBody},
    math::PI,
};

use crate::assets::LoadedAssets;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_walls);
    }
}

const WALL_HEIGHT: f32 = 50.0;
const WALL_WIDTH: f32 = 25.0;

fn spawn_walls(mut command: Commands, assets: Res<LoadedAssets>) {
    spawn_wall_column(
        &mut command,
        assets.wall.clone(),
        14,
        Vec3::new(-400.0, -350.0, 0.0),
    );
    spawn_wall_column(
        &mut command,
        assets.wall.clone(),
        14,
        Vec3::new(400.0, -350.0, 0.0),
    );
    spawn_ceil_row(
        &mut command,
        assets.wall.clone(),
        16,
        Vec3::new(-375.0, 340.0, 0.0),
    );
}

fn spawn_wall_column(
    command: &mut Commands,
    wall_texture: Handle<Image>,
    nbr: usize,
    start_pos: Vec3,
) {
    for index in 0..nbr {
        let mut pos = start_pos;
        pos.y += WALL_HEIGHT * index as f32;
        command.spawn((
            SpriteBundle {
                texture: wall_texture.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            Name::new("Wall"),
            RigidBody::Static,
            Collider::cuboid(WALL_WIDTH, WALL_HEIGHT),
        ));
    }
}

fn spawn_ceil_row(
    command: &mut Commands,
    wall_texture: Handle<Image>,
    nbr: usize,
    start_pos: Vec3,
) {
    for index in 0..nbr {
        let mut transform = Transform::from_translation(start_pos);
        transform.translation.x += WALL_HEIGHT * index as f32;
        transform.rotate_z(PI / 2.0);
        command.spawn((
            SpriteBundle {
                texture: wall_texture.clone(),
                transform,
                ..Default::default()
            },
            Name::new("Wall"),
            RigidBody::Static,
            Collider::cuboid(WALL_WIDTH, WALL_HEIGHT),
        ));
    }
}
