use kanal::{Sender, bounded};
use load_file::load_bytes;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use threadpool::Builder; 

use crate::node::page_node::Node;
use crate::message::{MessageOutEvent};

pub struct Galacticus {
    // The index of the node will be equivalent to it's id 
    pub nodes: Vec<Node>,
}

impl Galacticus {
    pub fn build() -> Self {
        let mut gal = Galacticus { nodes: vec![] };
        gal.create_nodes();
        gal

    }

    pub fn start(&self, destination: usize, sender: Sender<Arc<MessageOutEvent>>) {
        self.nodes[0].receive_message(&MessageOutEvent { from: 0, to: 0, path: Vec::with_capacity(7), destination, max_hops: 7}, sender);
    }

    pub fn listen(galacticus: Arc<RwLock<Galacticus>> ,destination: usize) -> bool {
        let pool = Builder::new()
            .thread_stack_size(1_000_000)
            .num_threads(6)
            .build();
        let binding = galacticus.clone();


        let mut total_missed = 0;
        let mut total_messages = 0;
        let mut last_decent_message = Instant::now();
        let mut has_arrived = false;

        let wait_delay = Duration::from_millis(100);

        let (sender, receiver) = bounded::<Arc<MessageOutEvent>>(1000);

        let clone = sender.clone();
        pool.execute(move ||
        {
            println!("Locking read.");
            let lock = binding.read().unwrap();
            println!("Locked read.");
            lock.start(destination, clone);
            println!("Unlocking read.");
        });


        let succes = loop {
            total_messages += 1;
            // if total_messages == 1_000_000 {
            //     println!("Total messages: {}. Took {:?} ms", total_messages, start.elapsed());
            //     break;
            // }

            let result = receiver.recv().unwrap();
            // println!("Received message");
            // thread::sleep(Duration::from_millis(1));

            if last_decent_message.elapsed() > wait_delay {
                println!("Current {}: Couldn't find any path.", destination);
                break false;
            }

            // drop the message if different iteration
            // Advance to the next iteration
            if result.destination != destination {
                continue;
            }

            if has_arrived {
                break true;
            }

            if result.to == result.destination {
                println!("Dest: {} -  {:?}", result.destination, result.path);
                // println!(" Total messages: {}. Took {:?} ms", total_messages, start.elapsed());
                has_arrived = true;
                continue
            }

            last_decent_message = Instant::now();



            let clone = galacticus.clone();

            let sender = sender.clone();
            pool.execute(move || {
                // println!("{}", thread::c);
                // let sender_id = result.path.last().unwrap();
                let a = clone.read().unwrap();
                a.nodes[result.to].receive_message(&result, sender);
            });

        };


        receiver.close();
        println!("Joining...");
        pool.join();
        println!("Joined...");

        return succes;
    }

    fn create_nodes(&mut self) {
        let dummy_json = load_bytes!("../../test_links.json");
        let v: Vec<Vec<usize>> = serde_json::from_slice(dummy_json).unwrap();

        let mut index = 0;
        for _ in v.as_slice() {
            let node = Node::new(index, vec![]);
            self.nodes.push(node);
            index += 1;
        }

        // Add neighbours 
        index = 0;
        let size = v.len();
        for val in v {
            for i in val {
                // just for testing. not needed on final json
                if i >= size {
                    continue;
                }

                let node = self.nodes[i].id;
                self.nodes[index].add_neighbour(node);
            }
            index += 1;
        }
    }
}
