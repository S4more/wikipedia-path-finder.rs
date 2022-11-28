use std::{time::{Instant, Duration}, path::PathBuf, sync::{Arc, Mutex, atomic::AtomicBool}};

use crate::galacticus::Galacticus;

use lazy_static::lazy_static;
use rayon::ThreadPoolBuilder;

mod node;
mod galacticus;
mod message;

use clap::{Parser, Command};
use rocket::{self, get, launch, routes, State};
use tokio::{time::sleep, runtime::Runtime};

struct MyState {
    galacticus: Galacticus,
}


#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long)]
    titles: String,

    #[clap(short, long)]
    page_relation: String,

    #[clap(short, long)]
    one_shot: Option<bool>,

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

#[rocket::main]
async fn main() {
    println!("here.");
    let args = Arguments::parse();
    ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    if args.one_shot.is_some() {
        handle_one_shot(args);
        return;
    }

    println!("{:?}", args);

    let gal = Galacticus::build(&args.titles, &args.page_relation);
    let gal_state = MyState { galacticus: gal } ;

    let _ = rocket::build().mount("/", routes![index]).manage(gal_state).launch().await.unwrap();
}

fn handle_one_shot(args: Arguments) {
    let galacticus: Galacticus = Galacticus::build(&args.titles, &args.page_relation);

    
    let mut missed = 0;
    let now = Instant::now();

    let mut current_percentage = Instant::now();
    for i in 25000..250_000 {
        let should_stop = Arc::new(AtomicBool::new(false));
        let found = galacticus.listen(0, i, 7, should_stop, Duration::from_secs(10));

        if found.is_none() {
            missed += 1;
            println!("Missed @ {}", i);
        }

        if i % 2500 == 0 {
            println!("{}% done. Current percentage time: {:?} Total Time: {:?}", (i / 2500) + 1, current_percentage.elapsed(), now.elapsed());
            current_percentage = Instant::now();
        }
    }
    print!("Total elapsed: {:?}. Missed: {}", now.elapsed(), missed);

    println!("Missed: {:?}", missed);
    println!("Took: {:?}", now.elapsed());

}
