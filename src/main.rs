use std::{time::{Instant, Duration}, sync::{Arc, atomic::AtomicBool}};

use crate::galacticus::Galacticus;

use rayon::ThreadPoolBuilder;

mod node;
mod galacticus;
mod message;

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
    mode: Mode,

    #[clap(short, long)]
    from: Option<usize>,

    #[clap(short, long)]
    destination: Option<usize>,

    #[clap(short, long)]
    number_of_hops: Option<usize>,
}

#[get("/<from>/<to>/<hops>")]
fn index(from: usize, to: usize, hops: u8, state: &State<MyState>) -> String {
    let atomic = Arc::new(AtomicBool::new(false));
    let result = state.galacticus.listen(from, to, hops.into(), atomic.clone(), Duration::from_secs(10));

    match result {
        Some(path) => format!("{:?}", path),
        None => "Couldn't find it".to_string(),
    }
}

async fn handle_server(args: Arguments) {
    let gal = Galacticus::build(&args.titles, &args.page_relation);
    let gal_state = MyState { galacticus: gal } ;
    let _ = rocket::build().mount("/", routes![index]).manage(gal_state).launch().await.unwrap();
}

#[rocket::main]
async fn main() {
    let args = Arguments::parse();
    ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    match args.mode {
        Mode::OneShot => one_shot(args),
        Mode::Range => handle_range(args),
        Mode::Server => handle_server(args).await,
    };
}

fn one_shot(args: Arguments) {
    let galacticus: Galacticus = Galacticus::build(&args.titles, &args.page_relation);

    let now = Instant::now();

    let should_stop = Arc::new(AtomicBool::new(false));
    let found = galacticus.listen(
        args.from.unwrap(),
        args.destination.unwrap(),
        args.number_of_hops.unwrap(),
        should_stop,
        Duration::from_secs(10),
        );

    match found {
        Some(path) => println!("path: {:?}", path),
        None => println!("Couldn't find it."),
    }

    println!("Took: {:?}", now.elapsed());
}

fn handle_range(args: Arguments) {
    let galacticus: Galacticus = Galacticus::build(&args.titles, &args.page_relation);

    let mut missed = 0;
    let now = Instant::now();

    // let mut current_percentage = Instant::now();
    for i in 0..100 {
        for j in 0..100 {
            if i == j || galacticus.nodes[i].neighbours.len() == 0 {
                continue;
            }
            let should_stop = Arc::new(AtomicBool::new(false));
            let found = galacticus.listen(i, j, 6, should_stop, Duration::from_secs(10));

            if found.is_none() {
                missed += 1;
                println!("Missed @ {}-{}", i, j);
            }

        }
    }
    print!("Total elapsed: {:?}. Missed: {}", now.elapsed(), missed);

    println!("Missed: {:?}", missed);
    println!("Took: {:?}", now.elapsed());

}
