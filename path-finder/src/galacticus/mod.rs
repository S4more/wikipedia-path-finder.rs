use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{
    fs::File,
    io::Read,
    ops::Index,
    sync::{atomic::Ordering::Relaxed, Arc, Mutex},
    time::{Duration, Instant},
};

use crate::node::page_node::Node;
use std::sync::atomic::AtomicBool;

use itertools::Itertools;

pub struct Galacticus {
    // The index of the node will be equivalent to it's id
    pub nodes: Vec<Node>,
    pub ordered_titles: Vec<String>,
}

lazy_static! {
    static ref CURRENT_PATH: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![]));
}

fn read_file(path: &String) -> Vec<u8> {
    let mut f = File::open(path).unwrap();
    let mut buffer = vec![0; f.metadata().unwrap().len() as usize];

    f.read_exact(&mut buffer).unwrap();

    buffer
}

impl Galacticus {
    pub fn build(
        path_to_ordered_titles: Option<String>,
        path_to_nodes: &String,
        path_to_incoming_nodes: &String,
    ) -> Self {
        println!("Reading file titles...");
        let mut titles = vec![];
        if let Some(path) = path_to_ordered_titles {
            let buffer = read_file(&path);
            titles = serde_json::from_slice(&buffer).unwrap();
        }

        let mut gal = Galacticus {
            nodes: vec![],
            ordered_titles: titles,
        };

        println!("Reading pages...");
        gal.create_nodes(path_to_nodes, path_to_incoming_nodes);
        println!("Ready to start!");

        gal
    }

    pub fn listen(
        &self,
        source: u32,
        destination: u32,
        max_hops: u32,
        should_stop: Arc<AtomicBool>,
        timeout: Duration,
    ) -> Option<Vec<u32>> {
        let local_node = &self.nodes[source as usize];

        if local_node.neighbours.is_empty() {
            println!("No neighbours. Can't do");
            return None;
        }

        let instant = Instant::now();

        let found = self.handle_branch(
            &local_node,
            source,
            destination,
            max_hops,
            0,
            should_stop,
            u32::MAX,
            &timeout,
            &instant,
        );

        // match found {
        //     None => self.log(source, destination),
        //     _ => {},
        // }

        if found.is_some() {
            let cur_path = self.get_and_clear_path();

            if cur_path.len() - 2 > max_hops as usize {
                println!("Too big");
                self.print_with_names(&cur_path);
            }
            Some(cur_path)
        } else {
            // println!("Starting reverse_lookup.");
            if instant.elapsed() > timeout {
                return self.reverse_lookup(local_node, destination, max_hops, timeout);
            }
            None
        }
    }

    pub fn reverse_lookup(
        &self,
        local_node: &Node,
        destination: u32,
        max_hops: u32,
        timeout: Duration,
    ) -> Option<Vec<u32>> {
        let reverse_size = 2;
        let in_the_way_nodes =
            self.get_neighbours_with_distance_of(&self.nodes[destination as usize], reverse_size);

        for node in in_the_way_nodes {
            let should_stop = Arc::new(AtomicBool::new(false));
            let now = Instant::now();
            let found = self.handle_branch(
                local_node,
                local_node.id,
                node.id,
                max_hops - reverse_size,
                0,
                should_stop,
                u32::MAX,
                &Duration::from_millis(25),
                &now,
            );

            if found.is_some() {
                let should_stop = Arc::new(AtomicBool::new(false));
                let mut first_half = self.get_and_clear_path();
                let found = self.handle_branch(
                    node,
                    node.id,
                    destination,
                    reverse_size + 1,
                    0,
                    should_stop,
                    u32::MAX,
                    &Duration::from_millis(100),
                    &now,
                );

                if found.is_some() {
                    let mut second_half = self.get_and_clear_path();

                    // if reverse_size + 1 < 3 {
                    //     second_half.pop();
                    // }

                    first_half.append(&mut second_half);
                    // self.print_with_names(&first_half);
                    return Some(first_half);
                }
                println!("Couldn't find shorter path.");
            }
            println!("Going for the next.");
            let mut cur_path_or = CURRENT_PATH.lock().unwrap();
            cur_path_or.clear();
        }

        return None;
    }

    fn get_and_clear_path(&self) -> Vec<u32> {
        let mut cur_path_or = CURRENT_PATH.lock().unwrap();
        let mut cur_path = cur_path_or.clone();
        cur_path_or.clear();
        cur_path.reverse();
        // self.print_with_names(&cur_path);
        cur_path
    }

    fn print_with_names(&self, vec: &Vec<u32>) {
        for l in vec {
            print!("({}, {}) - ", self.ordered_titles[*l as usize], l)
        }
        println!();
    }

    fn log(&self, source: u32, destination: u32) {
        println!(
            "Couldn't find path from {}  to: {}({})",
            self.ordered_titles[source as usize],
            self.ordered_titles[destination as usize],
            destination
        );
    }

    pub fn handle_branch(
        &self,
        node: &Node,
        source: u32,
        destination: u32,
        max_hops: u32,
        current_hop: u32,
        should_stop: Arc<AtomicBool>,
        from: u32,
        duration: &Duration,
        instant: &Instant,
    ) -> Option<u32> {
        if should_stop.load(Relaxed) {
            return None;
        }

        if instant.elapsed() > *duration {
            should_stop.store(true, Relaxed);
        }

        if current_hop >= max_hops {
            return None;
        }

        //... comment
        if node.id == destination {
            // println!("0. {} - ", node.id);
            return Some(node.id);
        }

        if node.has_neighbour(&destination) && !should_stop.load(Relaxed) {
            match CURRENT_PATH.try_lock() {
                Ok(mut lock) => {
                    if let Some(val) = lock.first() {
                        if val == &destination {
                            return None;
                        }
                    }
                    lock.push(destination);
                    lock.push(node.id);
                    should_stop.store(true, Relaxed);
                    return Some(node.id);
                }
                Err(_) => return None,
            }
            //     unwrap().push(node.id);
            // should_stop.store(true, Relaxed);
            // return Some(node.id);
        }

        if current_hop + 1 == max_hops {
            return None;
        }

        // Will happen only once
        if current_hop + 3 == max_hops {
            let found = node.neighbours.par_iter().find_any(|n| {
                let should_stop = should_stop.clone();
                let should_stop2 = should_stop.clone();

                if n == &&source || n == &&from {
                    return false;
                }

                let new_node = &self.nodes[**n as usize];
                if self
                    .handle_branch(
                        new_node,
                        source,
                        destination,
                        max_hops,
                        current_hop + 1,
                        should_stop,
                        node.id,
                        duration,
                        instant,
                    )
                    .is_some()
                {
                    should_stop2.store(true, Relaxed);
                    return true;
                }

                false
            });

            match found {
                // one edge case is if the current_hop + 3 == max_hops is the first possible scenario.
                // This won't add the started node to it.
                // In other words, when the max_hops is 3 and the current hops is 0, we will miss the
                // origin.
                Some(val) => {
                    if current_hop == 0  {
                        CURRENT_PATH.lock().unwrap().push(node.id);
                    }
                    return Some(*val)
                },
                None => return None,
            }
        }

        for node_id in &node.neighbours {
            let next_node = &self.nodes[*node_id as usize];
            let result = self.handle_branch(
                next_node,
                source,
                destination,
                max_hops,
                current_hop + 1,
                should_stop.clone(),
                node.id,
                duration,
                instant,
            );
            if result.is_some() {
                CURRENT_PATH.lock().unwrap().push(node.id);
                return result;
            }
        }

        None
    }

    fn create_nodes(&mut self, path_to_nodes: &String, path_to_incoming_nodes: &String) {
        let now = Instant::now();
        let buffer = read_file(path_to_nodes);

        let mut v: Vec<Vec<u32>> = serde_json::from_slice(&buffer).unwrap();

        println!("Took {:?}", now.elapsed());

        let now = Instant::now();
        println!("Creating nodes...");

        v.par_iter_mut()
            .enumerate()
            .map(|(index, neighbours)| {
                neighbours.sort();
                let ordered_neighbours = neighbours.clone();
                neighbours.shrink_to_fit();
                let r = Node::new(index as u32, ordered_neighbours, vec![]);
                neighbours.clear();
                r
            })
            .collect_into_vec(&mut self.nodes);

        // Decreases the amount of memory loaded at once.
        drop(v);
        drop(buffer);

        let buffer = read_file(path_to_incoming_nodes);
        let incoming_nodes: Vec<Vec<u32>> = serde_json::from_slice(&buffer).unwrap();

        self.nodes.par_iter_mut()
            .zip(incoming_nodes)
            .for_each(|(node, mut neighbours)| {
                node.neighbours.append(&mut neighbours)
            });

        println!("Took: {:?}", now.elapsed());
    }

    fn get_neighbours_with_distance_of<'a>(
        &'a self,
        node: &'a Node,
        distance: u32,
    ) -> Vec<&'a Node> {
        let now = Instant::now();
        let mut neighbours = vec![node];
        for _ in 0..distance {
            let local_neighbours = neighbours
                .par_iter()
                .map(|n| {
                    n.incoming_neighbours
                        .iter()
                        .map(|incoming| &self.nodes[*incoming as usize])
                        .collect()
                })
                .flat_map(|a: Vec<&Node>| a)
                .collect();

            neighbours = local_neighbours;
        }
        neighbours.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        // println!("Took: {:?}", now.elapsed());
        neighbours.into_iter().unique_by(|n| n.id).collect_vec()
    }

    pub fn get_id_from_title(&self, title: impl Into<String>) -> Option<usize> {
        let title: String = title.into();
        self.ordered_titles.iter().position(|t| t == &title)
    }
}
