use bevy::prelude::*;
use rand::random;

use crate::hash_grid::HashGrid;
#[derive(Component, Default)]
pub struct PhysicsObject {
    pub last_position: Vec2,
    pub current_position: Vec2,
}
pub struct WikiNode {
    title: String,
}

pub fn spawn_nodes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PhysicsObject {
            last_position: Vec2::new(0.0, 0.0),
            current_position: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                * 5.0,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: asset_server.load(format!("/random/{}.png", random::<u32>())),
            ..default()
        },
    ));
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
