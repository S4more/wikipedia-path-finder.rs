use std::sync::{Arc, Mutex, RwLock};

use crate::galacticus::Galacticus;

use lazy_static::lazy_static;

mod node;
mod galacticus;
mod message;

lazy_static! {
    // static ref GALACTICUS: Arc<RwLock<Galacticus>> = Arc::new(RwLock::new(Galacticus::build()));
}

fn main() {

    let mut current = 1;
    let mut missed = 0;
    let galacticus: Arc<RwLock<Galacticus>> = Arc::new(RwLock::new(Galacticus::build()));

    while current < 236 {
        let clone = galacticus.clone();
        // lock.start(current);
        // let gal = Arc::clone(&GALACTICUS);
        // let has_found = lock.listen(current);
        let has_found = Galacticus::listen(clone, current);

        if !has_found {
            missed += 1;
        }

        current += 1;
    }

}
