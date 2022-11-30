pub struct Node {
    pub id: usize,
    pub neighbours: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, neighbours: Vec<usize>) -> Self {
        Node {
            id,
            neighbours,
        }
    }

    pub fn has_neighbour(&self, id: &usize) -> bool {
        return self.neighbours.contains(id);
    }
}

