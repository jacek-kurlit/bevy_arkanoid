use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, RigidBody},
    math::PI,
};

use crate::{assets::LoadedAssets, ball::BallBounce};

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
        Vec3::new(-400.0, -20.0, 0.0),
        BallBounce::new(Vec2::new(-1.0, 1.0)),
    );
    spawn_wall_column(
        &mut command,
        assets.wall.clone(),
        14,
        Vec3::new(400.0, -20.0, 0.0),
        BallBounce::new(Vec2::new(-1.0, 1.0)),
    );
    spawn_ceil_row(
        &mut command,
        assets.wall.clone(),
        16,
        Vec3::new(0.0, 340.0, 0.0),
        BallBounce::new(Vec2::new(1.0, -1.0)),
    );
}

fn spawn_wall_column(
    command: &mut Commands,
    wall_texture: Handle<Image>,
    nbr: usize,
    start_pos: Vec3,
    ball_bounce: BallBounce,
) {
    let mut parent = command.spawn((
        SpriteBundle {
            transform: Transform::from_translation(start_pos),
            ..Default::default()
        },
        Name::new("Wall"),
        RigidBody::Static,
        Collider::cuboid(WALL_WIDTH, nbr as f32 * WALL_HEIGHT),
        ball_bounce,
    ));
    for index in 0..nbr {
        let pos = Vec3::new(0.0, -325.0 + WALL_HEIGHT * index as f32, 0.0);
        parent.with_children(|cmd| {
            cmd.spawn((SpriteBundle {
                texture: wall_texture.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },));
        });
    }
}

fn spawn_ceil_row(
    command: &mut Commands,
    wall_texture: Handle<Image>,
    nbr: usize,
    start_pos: Vec3,
    ball_bounce: BallBounce,
) {
    let mut transform = Transform::from_translation(start_pos);
    transform.rotate_z(PI / 2.0);
    let mut parent = command.spawn((
        SpriteBundle {
            transform,
            ..Default::default()
        },
        Name::new("Wall Ceiling"),
        RigidBody::Static,
        Collider::cuboid(WALL_WIDTH, nbr as f32 * WALL_HEIGHT),
        ball_bounce,
    ));
    for index in 0..nbr {
        let pos = Vec3::new(0.0, -375.0 + WALL_HEIGHT * index as f32, 0.0);
        parent.with_children(|cmd| {
            cmd.spawn((SpriteBundle {
                texture: wall_texture.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },));
        });
    }
}
