mod bufferpool;
mod utils;
mod page_interpretation;
mod disk;

use std::{thread::{self}, sync::{Arc, Mutex}};

use crate::bufferpool::{Pool, EvictionStrategy, Page};

fn main() {
    let strat: Mutex<Box<dyn EvictionStrategy>> = Mutex::new(Box::new(bufferpool::eviction::LruK::new(10, 2)));
    let pool = Arc::new(Pool::new(10, strat));
    let mut threads = Vec::new();
    for _ in 1..11 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            if let Some(result) = pool_clone.new_page(){
                let id = result.0;
                let mut write_guard = result.1.write();
                let cur = write_guard[0];
                println!("got guard for page {}, contents is {:?}", id, cur);
                *write_guard = Page::from([cur + 1 as u8; 4096]);
                drop(write_guard);
                drop(result.1);
            }
            else {
                eprintln!("could not make page");
            }
        });
        threads.push(handle);
    } 

    for t in threads {
        t.join().unwrap();
    }
}
