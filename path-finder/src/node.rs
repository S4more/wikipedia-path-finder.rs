pub struct Node {
    pub id: u32,
    pub neighbours: Vec<u32>,
    pub incoming_neighbours: Vec<u32>,
}

impl Node {
    pub fn new(id: u32, neighbours: Vec<u32>, incoming_neighbours: Vec<u32>) -> Self {
        Node {
            id,
            neighbours,
            incoming_neighbours,
        }
    }

    pub fn has_neighbour(&self, id: &u32) -> bool {
        self.neighbours.contains(id)
    }
}
