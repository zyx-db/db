mod bufferpool;
mod utils;
mod page_interpretation;
use std::{thread::{self}, sync::{Arc, Mutex}};

// use bufferpool::eviction;

use crate::bufferpool::{Pool, EvictionStrategy, Page};

fn main() {
    let strat: Mutex<Box<dyn EvictionStrategy>> = Mutex::new(Box::new(bufferpool::eviction::LruK::new(10, 2)));
    let pool = Arc::new(Pool::new(10, strat));
    let mut threads = Vec::new();
    for i in 1..11 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let guard = pool_clone.get_page(i % 2);
            let mut write_guard = guard.write();
            let cur = write_guard[0];
            println!("got guard for page {}, contents is {:?}", i % 2, cur);
            *write_guard = Page::from([cur + 1 as u8; 4096]);
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
