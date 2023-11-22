mod bufferpool;
mod utils;
use std::{thread::{self, sleep}, sync::Arc, time::Duration};

use crate::bufferpool::{Pool, EvictionStrategy};

fn main() {
    let pool = Arc::new(Pool::new(2, EvictionStrategy::LruK));
    let mut threads = Vec::new();
    for i in 0..10 {
        // let guard = pool.get(1)
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let guard = pool_clone.get_page(i);
            sleep(Duration::from_secs(1));
            drop(guard);
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}
