use std::{time::Instant, path::PathBuf};

use crate::galacticus::Galacticus;

use lazy_static::lazy_static;
use rayon::ThreadPoolBuilder;

mod node;
mod galacticus;
mod message;

use clap::{Parser, Command};

lazy_static! {
    // static ref GALACTICUS: Arc<RwLock<Galacticus>> = Arc::new(RwLock::new(Galacticus::build()));
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

fn main() {
    let args = Arguments::parse();

    println!("{:?}", args);

    ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let mut missed = 0;
    let galacticus: Galacticus = Galacticus::build(&args.titles, &args.page_relation);

    let now = Instant::now();
    println!("{}", galacticus.ordered_titles[37971]);

    for i in 0..1 {
        let now = Instant::now();
        for j in 37971..100_000 {
            if i == j || galacticus.nodes[i].neighbours.len() == 0 {
                continue;
            }
            // let clone = galacticus.clone();
            let mut found;
            for depth in 7..20 {
                found = galacticus.listen(i, j, depth);
                if found.is_some() {
                    break;
                } else {
                    println!("Couldn't find. Increasing depth to {}", depth + 1);
                }
            }
            println!("Found: {}. Took: {:?}", j, now.elapsed());
        }
        println!("Found: {}. Took: {:?}", i, now.elapsed());
    }
    println!("Missed: {:?}", missed);
    println!("Took: {:?}", now.elapsed());
    // println!("Took: {:?}", now.elapsed());
}
