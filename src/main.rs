mod bufferpool;
mod utils;
use std::{thread::{self}, sync::{Arc, Mutex}};

use bufferpool::eviction;

use crate::bufferpool::{Pool, EvictionStrategy, Page};

fn main() {
    let strat: Mutex<Box<dyn EvictionStrategy>> = Mutex::new(Box::new(bufferpool::eviction::LruK::new(2, 2)));
    let pool = Arc::new(Pool::new(2, strat));
    let mut threads = Vec::new();
    for i in 1..2 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let guard = pool_clone.get_page(0);
            let mut write_guard = guard.write();
            println!("got guard, contents is {:?}", write_guard[0]);
            *write_guard = Page::from([i as u8; 4096]);
            // sleep(Duration::from_secs(1));
            drop(write_guard);
            drop(guard);
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}
