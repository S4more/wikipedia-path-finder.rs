use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PhysicsObject {
    pub last_position: Vec2,
    pub current_position: Vec2,
}

pub struct WikiNode {
    title: String,
}

pub fn add_nodes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PhysicsObject {
            last_position: Vec2::new(0.0, 0.0),
            current_position: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5),
        },
        SpriteBundle {
            transform: Transform::from_xyz(100., 0., 0.),
            texture: asset_server.load("test.png"),
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
    let gravity = Vec2 { x: 0.0, y: -0.01 };

    for mut node in query.iter_mut() {
        let last = node.current_position.clone();
        let delta = (node.current_position - node.last_position) * drag;

        node.current_position += gravity;
        node.current_position += delta;
        node.last_position = last;
    }
}

// pub fn setup_nodes(mut commands: Commands) {
// commands.spawn(Node).with(Position)
// }
