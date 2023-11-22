mod bufferpool;
mod utils;
use std::{thread::{self, sleep}, sync::Arc, time::Duration};

use crate::bufferpool::{Pool, EvictionStrategy, Page};

fn main() {
    let pool = Arc::new(Pool::new(2, EvictionStrategy::LruK));
    let mut threads = Vec::new();
    for i in 1..11 {
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
