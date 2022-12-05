use bevy::prelude::*;
use rand::random;

use crate::{connection::create_connection, hash_grid::HashGrid};
#[derive(Component, Default)]
pub struct PhysicsObject {
    pub last_position: Vec2,
    pub current_position: Vec2,
}

pub struct WikiNode {
    title: String,
}

#[derive(Resource)]
pub struct SpawnInterval {
    elapsed_seconds: f32,
    pub threshold_seconds: f32,
}
impl SpawnInterval {
    pub fn should_spawn(&mut self, elapsed: f32) -> bool {
        self.elapsed_seconds += elapsed;
        if self.elapsed_seconds >= self.threshold_seconds {
            self.elapsed_seconds -= self.threshold_seconds;
            return true;
        } else {
            return false;
        }
    }
    pub fn new(interval: f32) -> Self {
        Self {
            elapsed_seconds: 0.0,
            threshold_seconds: interval,
        }
    }
}

pub fn spawn_nodes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_interval: ResMut<SpawnInterval>,
    nodes: Query<(&PhysicsObject, Entity)>,
) {
    if spawn_interval.should_spawn(time.delta_seconds()) {
        let nodes: Vec<(&PhysicsObject, Entity)> = nodes.into_iter().collect();
        let index = random::<usize>() % nodes.len();

        let (node_pos, node_id) = nodes[index];

        let new_id = commands
            .spawn(create_node(
                asset_server.load(format!("/random/{}.png", random::<u32>())),
                node_pos.current_position,
            ))
            .id();

        commands.spawn(create_connection(node_id, new_id));
    }
}

pub fn create_node(texture: Handle<Image>, position: Vec2) -> (PhysicsObject, SpriteBundle) {
    (
        PhysicsObject {
            last_position: position.clone(),
            current_position: position
                + Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5) * 2.0,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.),
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(23., 23.)),
                ..default()
            },
            ..default()
        },
    )
}

pub fn sprite_position_update(mut query: Query<(&PhysicsObject, &mut Transform)>) {
    for obj in query.iter_mut() {
        let (physics, mut transform) = obj;
        transform.translation.x = physics.current_position.x;
        transform.translation.y = physics.current_position.y;
    }
}

pub fn step_nodes(mut query: Query<&mut PhysicsObject>) {
    let drag = 0.98;

    for mut node in query.iter_mut() {
        let last = node.current_position.clone();
        let delta = (node.current_position - node.last_position) * drag;

        node.current_position += delta;
        node.last_position = last;
    }
}

pub fn node_repulsion(mut query: Query<(&mut PhysicsObject, Entity)>, grid: Res<HashGrid>) {
    let iter = query.iter();
    let collisions: Vec<(Entity, Entity)> = grid.get_collisions(iter);

    for (a, b) in collisions {
        if let Ok([(mut a, _), (mut b, _)]) = query.get_many_mut([a, b]) {
            let d = a.current_position.distance_squared(b.current_position);
            let norm = a.current_position - b.current_position;
            if d < 32.0 * 32.0 {
                b.current_position -= norm * (1.0 / d);
                a.current_position += norm * (1.0 / d);
            }
        }
    }
}
