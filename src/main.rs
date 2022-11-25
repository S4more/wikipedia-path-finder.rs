use std::time::Instant;

use crate::galacticus::Galacticus;

use lazy_static::lazy_static;
use rayon::ThreadPoolBuilder;

mod node;
mod galacticus;
mod message;

lazy_static! {
    // static ref GALACTICUS: Arc<RwLock<Galacticus>> = Arc::new(RwLock::new(Galacticus::build()));
}

fn main() {
    ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let mut missed = 0;
    let galacticus: Galacticus = Galacticus::build();

    let now = Instant::now();

    // for i in 1..250_000  {
    //     let result = galacticus.listen(0, i, 8);
    //     if result.is_none() {
    //         missed += 1;
    //     }
    // }

    println!("Missed: {}", missed);

    for i in 0..100 {
        let now = Instant::now();
        for j in 0..100 {
            if i == j || galacticus.nodes[i].neighbours.len() == 0 {
                continue;
            }
            // let clone = galacticus.clone();
            let has_found = galacticus.listen(i, j, 7);
            if has_found.is_none() {
                missed += 1;
                println!("Missed.");
            }
        }
        println!("Found: {}. Took: {:?}", i, now.elapsed());
    }
    println!("Took: {:?}", now.elapsed());
    // println!("Took: {:?}", now.elapsed());
}
