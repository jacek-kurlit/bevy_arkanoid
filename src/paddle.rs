use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, RigidBody};

use crate::assets::LoadedAssets;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        //NOTE: PostStartup because ion start up we are loding assets!
        app.add_systems(PostStartup, spawn_paddle)
            .add_systems(Update, move_paddle);
    }
}

pub const PADDLE_STARTING_POS: Vec3 = Vec3::new(0.0, -300.0, 0.0);
const PADDLE_SPEED: f32 = 350.0;

#[derive(Debug, Component)]
struct Paddle;

fn spawn_paddle(mut commands: Commands, loaded_assets: Res<LoadedAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: loaded_assets.paddle.clone(),
            transform: Transform::from_translation(PADDLE_STARTING_POS),
            ..Default::default()
        },
        Paddle,
        RigidBody::Kinematic,
        Collider::cuboid(80.0, 20.0),
    ));
}

fn move_paddle(
    mut query: Query<&mut Transform, With<Paddle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    if input.pressed(KeyCode::A) {
        transform.translation.x -= PADDLE_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += PADDLE_SPEED * time.delta_seconds();
    }
}
