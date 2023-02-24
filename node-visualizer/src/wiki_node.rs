use bevy::prelude::*;

use crate::{connection::create_connection, hash_grid::HashGrid, PathResource};
#[derive(Component, Default)]
pub struct PhysicsObject {
    pub last_position: Vec2,
    pub current_position: Vec2,
}
pub fn setup_nodes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path: Res<PathResource>,
) {
    let titles: Vec<String> = path.0.split(',').map(|v| v.to_string()).collect();
    println!("{:?}", titles);

    let ids: Vec<Entity> = titles
        .iter()
        .map(|title| {
            commands
                .spawn(create_node(
                    asset_server.load(format!("/image/{}.png", title)),
                    Vec2::new(0., 0.),
                ))
                .id()
        })
        .collect();

    for i in 0..ids.len() - 1 {
        commands.spawn(create_connection(ids[i], ids[i + 1]));
    }
}

pub fn create_node(texture: Handle<Image>, position: Vec2) -> (PhysicsObject, SpriteBundle) {
    (
        PhysicsObject {
            last_position: position,
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
        let last = node.current_position;
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
