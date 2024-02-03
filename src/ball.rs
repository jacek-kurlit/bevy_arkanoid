use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, LinearVelocity, RigidBody},
    plugins::collision::contact_reporting::CollisionStarted,
};

use crate::{assets::LoadedAssets, bricks::Brick, paddle::PADDLE_STARTING_POS};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball)
            .add_systems(Update, (start_ball, print_collisions));
    }
}

#[derive(Component, Debug, Default)]
struct Ball {
    moving: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BallBounce(Vec2);

impl BallBounce {
    pub fn new(vec: Vec2) -> Self {
        Self(vec)
    }
}

const BALL_OFFSET_FROM_PADDLE: f32 = 30.0;
const BALL_SPEED: f32 = 300.0;

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
        Ball::default(),
        RigidBody::Kinematic,
        Collider::ball(15.0),
    ));
}

fn start_ball(mut query: Query<(&mut LinearVelocity, &mut Ball)>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let Ok((mut velocity, mut ball)) = query.get_single_mut() else {
            return;
        };
        if ball.moving {
            return;
        }
        ball.moving = true;
        velocity.y = BALL_SPEED;
        velocity.x = BALL_SPEED;
    }
}

fn print_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
    brick_query: Query<&Brick>,
    bounce_query: Query<&BallBounce>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        info!("Entities {:?} and {:?} are colliding", entity1, entity2);
        if let Ok(ball_velocity) = ball_query.get_mut(*entity1) {
            handle_ball_collision(
                &bounce_query,
                entity2,
                ball_velocity,
                &brick_query,
                &mut commands,
            );
        }
        if let Ok(ball_velocity) = ball_query.get_mut(*entity2) {
            handle_ball_collision(
                &bounce_query,
                entity1,
                ball_velocity,
                &brick_query,
                &mut commands,
            )
        }
    }
}

fn handle_ball_collision(
    bounce_query: &Query<'_, '_, &BallBounce>,
    entity2: &Entity,
    mut ball_velocity: Mut<'_, LinearVelocity>,
    brick_query: &Query<'_, '_, &Brick>,
    commands: &mut Commands<'_, '_>,
) {
    let Ok(ball_bounce) = bounce_query.get(*entity2) else {
        return;
    };
    ball_velocity.x = ball_velocity.x.signum() * ball_bounce.0.x * BALL_SPEED;
    ball_velocity.y = ball_velocity.y.signum() * ball_bounce.0.y * BALL_SPEED;
    if brick_query.get_component::<Brick>(*entity2).is_ok() {
        commands.entity(*entity2).despawn_recursive();
    }
}
