use bevy::prelude::*;

use crate::{assets::LoadedAssets, paddle::PADDLE_STARTING_POS};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball);
    }
}

const BALL_OFFSET_FROM_PADDLE: f32 = 30.0;

fn spawn_ball(mut commands: Commands, assets: Res<LoadedAssets>) {
    let mut transform = Transform::from_translation(PADDLE_STARTING_POS);
    transform.translation.y += BALL_OFFSET_FROM_PADDLE;
    commands.spawn((
        SpriteBundle {
            texture: assets.ball.clone(),
            transform,
            ..Default::default()
        },
        Name::new("Ball"),
    ));
}
