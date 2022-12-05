use crate::wiki_node::PhysicsObject;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::{
    DrawMode, FillMode, GeometryBuilder, Path, ShapePath, StrokeMode,
};
use bevy_prototype_lyon::shapes::Line;

#[derive(Component)]
pub struct Connection(pub Entity, pub Entity);

pub fn step_connections(
    mut nodes: Query<(&mut PhysicsObject, Entity)>,
    mut connections: Query<(&Connection, &mut Path)>,
) {
    for (Connection(from, to), mut path) in connections.iter_mut() {
        if let Ok([(mut from_node, _), (mut to_node, _)]) = nodes.get_many_mut([*from, *to]) {
            let from = from_node.current_position;
            let to = to_node.current_position;

            let multiplier = (from.distance_squared(to) - 90.0).max(0.0) / 1000000.0;

            let norm = ((from - to) * multiplier).clamp_length_max(5.0);

            from_node.current_position -= norm;
            to_node.current_position += norm;
            *path = ShapePath::build_as(&Line(from, to));
        }
    }
}

pub fn create_connection(from: Entity, to: Entity) -> (Connection, ShapeBundle) {
    let shape = Line(Vec2::default(), Vec2::default());

    let geometry = GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLACK),
            outline_mode: StrokeMode::new(Color::BLACK, 4.0),
        },
        Transform::default(),
    );

    (Connection(from, to), geometry)
}
