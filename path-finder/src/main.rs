use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

use crate::galacticus::Galacticus;

use rayon::ThreadPoolBuilder;

mod galacticus;
mod message;
mod node;

use clap::Parser;
use rocket::{self, get, routes, State};

struct MyState {
    galacticus: Galacticus,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    OneShot,
    Range,
    Server,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Server
    }
}

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long)]
    titles: String,

    #[clap(short, long)]
    page_relation: String,

    #[clap(short, long)]
    incoming_neighbours: String,

    #[clap(short, long)]
    mode: Mode,

    #[clap(short, long)]
    from: Option<u32>,

    #[clap(short, long)]
    destination: Option<u32>,

    #[clap(short, long)]
    number_of_hops: Option<u32>,
}

fn search_base(from: u32, to: u32, hops: u8, state: &State<MyState>) -> Option<Vec<u32>> {
    let atomic = Arc::new(AtomicBool::new(false));

    let result = state
        .galacticus
        .listen(from, to, hops.into(), atomic, Duration::from_secs(10));

    match result {
        Some(mut path) => {
            if hops == 3 {
                let mut new_path = vec![from];
                new_path.append(&mut path);
                path = new_path;
            }

            Some(path)
        }
        None => None,
    }
}

#[get("/id/<from>/<to>/<hops>")]
fn search_id(from: u32, to: u32, hops: u8, state: &State<MyState>) -> String {
    if let Some(result) = search_base(from, to, hops, state) {
        format!("{:?}", result)
    } else {
        "Couldn't find it".to_string()
    }
}

#[get("/title/<from>/<to>/<hops>")]
fn search_title(from: &str, to: &str, hops: u8, state: &State<MyState>) -> String {
    let from = state.galacticus.get_id_from_title(from);
    let to = state.galacticus.get_id_from_title(to);

    if let (Some(from), Some(to)) = (from, to) {
        if let Some(result) = search_base(from as u32, to as u32, hops, state) {
            let titles: Vec<String> = result
                .iter()
                .map(|item| state.galacticus.ordered_titles[*item as usize].clone())
                .collect();
            format!("{titles:?}")
        } else {
            "Couldn't find it".to_string()
        }
    } else {
        "Error: Invalid title".to_string()
    }
}

async fn handle_server(args: Arguments) {
    let gal = Galacticus::build(&args.titles, &args.page_relation, &args.incoming_neighbours);
    let gal_state = MyState { galacticus: gal };
    let _ = rocket::build()
        .mount("/", routes![search_id, search_title])
        .manage(gal_state)
        .launch()
        .await
        .unwrap();
}

#[rocket::main]
async fn main() {
    let args = Arguments::parse();
    ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    match args.mode {
        Mode::OneShot => one_shot(args),
        Mode::Range => handle_range(args),
        Mode::Server => handle_server(args).await,
    };
}

fn one_shot(args: Arguments) {
    let galacticus: Galacticus =
        Galacticus::build(&args.titles, &args.page_relation, &args.incoming_neighbours);

    let now = Instant::now();

    let should_stop = Arc::new(AtomicBool::new(false));
    let found = galacticus.listen(
        args.from.unwrap(),
        args.destination.unwrap(),
        args.number_of_hops.unwrap(),
        should_stop,
        Duration::from_millis(0),
    );

    match found {
        Some(path) => println!("path: {:?}", path),
        None => println!("Couldn't find it."),
    }

    println!("Took: {:?}", now.elapsed());
}

fn handle_range(args: Arguments) {
    let galacticus: Galacticus =
        Galacticus::build(&args.titles, &args.page_relation, &args.incoming_neighbours);

    let mut missed = 0;
    let now = Instant::now();

    // let mut current_percentage = Instant::now();
    for i in 0..1 {
        let now = Instant::now();
        for j in 800_000..900_000 {
            if i == j || galacticus.nodes[i].neighbours.is_empty() {
                continue;
            }
            let now = Instant::now();
            let should_stop = Arc::new(AtomicBool::new(false));
            let found = galacticus.listen(
                i as u32,
                j as u32,
                6,
                should_stop,
                Duration::from_millis(10),
            );

            if found.is_none() {
                missed += 1;
                println!("Missed @ {}-{}", i, j);
            }
            println!("{} {:?}", j, now.elapsed());
        }
        println!("Moved to {} iteration. Took: {:?}", i, now.elapsed());
    }
    print!("Total elapsed: {:?}. Missed: {}", now.elapsed(), missed);

    println!("Missed: {:?}", missed);
    println!("Took: {:?}", now.elapsed());
}
