use lazy_static::lazy_static;
use load_file::load_bytes;
use std::sync::{Arc, atomic::Ordering::Relaxed, Mutex};
use rayon::prelude::*;

use crate::node::page_node::Node;

use std::sync::atomic::AtomicBool;

pub struct Galacticus {
    // The index of the node will be equivalent to it's id 
    pub nodes: Vec<Node>,
    pub ordered_titles: Vec<String>,
}

lazy_static! {
    static ref CURRENT_PATH: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));
}

impl Galacticus {
    pub fn build() -> Self {

        let json = load_bytes!("../../ordered_titles/ordered_titles.json");
        let titles : Vec<String> = serde_json::from_slice(json).unwrap();

        let mut gal = Galacticus { nodes: vec![], ordered_titles: titles};

        gal.create_nodes();
        gal

    }

    pub fn listen(&self, source: usize, destination: usize, max_hops: usize) -> Option<Vec<usize>>{
        let local_node = &self.nodes[source];
        let found = self.handle_branch(&local_node, source, destination, max_hops, 0, Arc::new(AtomicBool::new(false)));

        match found {
            // Some(expr) => println!("Found path to: {}", read_lock.ordered_titles[destination]),
            None => self.log(source, destination),
            _ => {},
        }


        if found.is_some() {
            let mut cur_path_or = CURRENT_PATH.lock().unwrap();
            let mut cur_path = cur_path_or.clone();
            cur_path_or.clear();
            cur_path.reverse();
            cur_path.push(destination);
            // self.print_with_names(&cur_path);
            Some(cur_path)
        } else {
            None
        }
        // return succes;
    }

    fn print_with_names(&self, vec: &Vec<usize>) {
        for l in vec {
            print!("({}, {}) - ", self.ordered_titles[*l], l)
        }
        println!("");
    }

    fn log(&self, source: usize, destination: usize) {
        println!("Couldn't find path from {}  to: {}", self.ordered_titles[source], self.ordered_titles[destination]);
        // println!("The source has the following neighbours: ");
        //
        // for node in &self.nodes[source].neighbours {
        //     println!("{}", self.ordered_titles[*node]);
        // }

    }

    pub fn handle_branch(&self, 
                         node: &Node,
                         source: usize,
                         destination: usize,
                         max_hops: usize,
                         current_hop: usize,
                         should_stop: Arc<AtomicBool>
                         ) -> Option<usize>{

        if should_stop.load(Relaxed) {
            return None;
        }

        //... comment
        if node.id == destination {
            // println!("0. {} - ", node.id);
            return Some(node.id);
        }

        if current_hop + 1 == max_hops {
            if node.has_neighbour(&destination) {
                CURRENT_PATH.lock().unwrap().push(node.id);
                return Some(node.id);
            }  else {
                return None;
            }
        }
        
        // Will happen only once
        if current_hop + 3 == max_hops {

            let found = node.neighbours
                .par_iter()
                .find_any(|n| {
                    let should_stop = should_stop.clone();
                    let should_stop2 = should_stop.clone();

                    let node = &self.nodes[**n];
                    if self.handle_branch(&node, source, destination, max_hops, current_hop + 1, should_stop).is_some() {
                        should_stop2.store(true, Relaxed);
                        return true;
                    }

                    false
                });

            match found {
                Some(val) => return Some(*val),
                None => return None,
            }
        }

        if current_hop >= max_hops {
            return None;
        }

        for node_id in &node.neighbours  {
            let next_node = &self.nodes[*node_id];
            let result = self.handle_branch(&next_node, source, destination, max_hops, current_hop + 1, should_stop.clone());
            if result.is_some() {
                // println!("{} -> {} -> ",  node.id, next_node.id);
                // println!("[{}-{}]: This should be the last one: {}", current_hop, node.id, result.unwrap());
                CURRENT_PATH.lock().unwrap().push(node.id);
                return result;
            }
        }

        return None;
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
        for mut val in v {
            val.sort();
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
