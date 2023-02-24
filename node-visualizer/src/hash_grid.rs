use bevy::{
    prelude::{Entity, Resource},
    utils::{hashbrown::HashMap, HashSet},
};

use crate::wiki_node::PhysicsObject;

// This implementation assumes that the grid size is at least
// as small as a single containing object's bounding box

#[derive(Resource)]
pub struct HashGrid {
    pub grid_size: f32,
}

type PositionHash = (i32, i32);
type BoundHash = (PositionHash, PositionHash, PositionHash, PositionHash);

impl HashGrid {
    pub fn get_collisions<'a, I>(&'a self, objs: I) -> Vec<(Entity, Entity)>
    where
        I: Iterator<Item = (&'a PhysicsObject, Entity)>,
    {
        let mut builder = GridBuilder::new(self);

        for item in objs.into_iter() {
            builder.insert_obj(item)
        }
        builder.get_all_possible_collisions()
    }
}

pub struct GridBuilder<'a> {
    hash_map: HashMap<(i32, i32), HashSet<Entity>>,
    grid: &'a HashGrid,
}

impl<'a> GridBuilder<'a> {
    pub fn new(grid: &'a HashGrid) -> Self {
        Self {
            hash_map: HashMap::new(),
            grid,
        }
    }

    pub fn hash_bounds(&mut self, obj: &PhysicsObject) -> BoundHash {
        let half_size = self.grid.grid_size / 2.0;
        let px = obj.current_position.x;
        let py = obj.current_position.y;
        (
            self.hash_position(px - half_size, py - half_size),
            self.hash_position(px - half_size, py + half_size),
            self.hash_position(px + half_size, py - half_size),
            self.hash_position(px + half_size, py + half_size),
        )
    }

    pub fn hash_position(&self, x: f32, y: f32) -> PositionHash {
        (
            (x / self.grid.grid_size) as i32,
            (y / self.grid.grid_size) as i32,
        )
    }

    pub fn insert_obj(&mut self, object: (&'a PhysicsObject, Entity)) {
        let (a, b, c, d) = self.hash_bounds(object.0);

        self.insert_pos(a, object);
        self.insert_pos(b, object);
        self.insert_pos(c, object);
        self.insert_pos(d, object);
    }

    pub fn insert_pos(&mut self, pos: PositionHash, object: (&'a PhysicsObject, Entity)) {
        let (_, entity) = object;

        if let Some(arr) = self.hash_map.get_mut(&pos) {
            arr.insert(entity);
        } else {
            self.hash_map.insert(pos, HashSet::from([entity]));
        }
    }

    pub fn get_all_possible_collisions(&self) -> Vec<(Entity, Entity)> {
        let mut collisions: HashSet<(Entity, Entity)> = HashSet::new();

        for bucket in self
            .hash_map
            .values()
            .map(|v| v.iter().collect::<Vec<&Entity>>())
        {
            for i in 0..bucket.len() {
                for j in i + 1..bucket.len() {
                    collisions.insert((*bucket[i], *bucket[j]));
                }
            }
        }

        collisions.into_iter().collect()
    }
}
