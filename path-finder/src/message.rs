// impl Drop for Message {
//     fn drop(&mut self) {
//         // println!("Dropping. {:?}", self.path);
//     }
// }

#[derive(Clone)]
pub struct MessageOutEvent {
    pub to: usize,
    pub from: usize,
    // pub path: Vec<usize>,
    pub destination: usize,
    pub current_hop: usize,
    pub max_hops: usize,
}

