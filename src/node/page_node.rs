use std::{thread::{self, Thread}, time::Duration, panic::resume_unwind, sync::Arc};

use kanal::Sender;

use crate::message::{MessageOutEvent};

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

    pub fn add_neighbour(&mut self, neighbour: usize) {
        // if neighbour == 33 {
        //     println!("[{}] adding neighbour {}", self.id, neighbour);
        // }
        self.neighbours.push(neighbour);
    }

    pub fn has_neighbour(&self, id: &usize) -> bool {
        for neighbour_id in &self.neighbours {
            if neighbour_id == id {
                return true;
            }
        }

        return false;
    }

    // pub fn receive_message(&self, mut message: &MessageOutEvent, sender: Sender<Arc<MessageOutEvent>>) {
    //
    //     // if message.path.len() > message.max_hops {
    //     //     message.path.clear();
    //     //     return;
    //     // }
    //     //
    //     if message.destination == self.id {
    //         println!("Arrived. Destination: {:?}", message.destination);
    //     }
    //
    //     // Check for loops
    //     // for node in &message.path {
    //     //     if node == &self.id {
    //     //         // println!("Dropping loop. {:?}", message.destination);
    //     //         return;
    //     //     }
    //     // }
    //
    //
    //     // self.mutate_message(message);
    //     // message.path.to_owned()
    //     // println!("Path: {:?}", message.path);
    //
    //     let mut clone = message.clone();
    //     clone.from = self.id;
    //     clone.to = *neighbour;
    //     clone.current_hop += 1;
    //
    //     for neighbour in self.neighbours.iter() {
    //         if neighbour == &message.from {
    //             // We don't want to send the message back to the sender.
    //             continue;
    //         }
    //
    //
    //
    //         match sender.send(clone) {
    //             Ok(_) => {
    //             },
    //             Err(_) => { break },
    //         }
    //
    //         // self.galacticus.forward_message(self.id, neighbour.id, message.clone());
    //     }
    // }

    fn mutate_message(&self, message: &mut MessageOutEvent) {
        // message.path.push(self.id);
        // message.max_hops += 1;
    }

}

