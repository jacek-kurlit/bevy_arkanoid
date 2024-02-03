use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_2d::components::{Collider, RigidBody};

use crate::{assets::LoadedAssets, ball::BallBounce};

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bricks);
    }
}

#[derive(Debug, Component)]
#[allow(dead_code)]
pub struct Brick {
    hp: usize,
    point: usize,
}

impl Brick {
    fn new(hp: usize, point: usize) -> Self {
        Self { hp, point }
    }
}

const NUMBER_OF_COLUMNS: usize = 7;
const BRICK_WIDTH: f32 = 60.0;
const BRICK_HEIGHT: f32 = 30.0;
const BRICK_OFFSET: f32 = BRICK_WIDTH + 0.5;
const ROW_OFFSET: f32 = BRICK_HEIGHT + 0.5;
const BRICKS_STARTING_POS: Vec3 = Vec3::new(-200.0, 100.0, 0.0);

fn spawn_bricks(mut commands: Commands, assets: Res<LoadedAssets>) {
    let mut parent = commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(BRICKS_STARTING_POS),
            ..Default::default()
        },
        Name::new("Bricks"),
    ));
    spawn_bricks_row(0, "Blue Brick", assets.blue_brick.clone(), &mut parent);
    spawn_bricks_row(1, "Red Brick", assets.red_brick.clone(), &mut parent);
    spawn_bricks_row(2, "Pink Brick", assets.pink_brick.clone(), &mut parent);
    spawn_bricks_row(3, "Yellow Brick", assets.yellow_brick.clone(), &mut parent);
}

fn spawn_bricks_row(
    row_number: usize,
    name: &'static str,
    texture: Handle<Image>,
    parent: &mut EntityCommands,
) {
    for column in 0..NUMBER_OF_COLUMNS {
        let pos = Vec3::new(
            BRICK_OFFSET * column as f32,
            row_number as f32 * ROW_OFFSET,
            0.0,
        );
        parent.with_children(|cmd| {
            cmd.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                },
                Brick::new(row_number, row_number * 40),
                Name::new(name),
                RigidBody::Static,
                Collider::cuboid(BRICK_WIDTH, BRICK_HEIGHT),
                BallBounce::new(Vec2::new(1.0, -1.0)),
            ));
        });
    }
}
