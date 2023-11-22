use std::{collections::HashMap, sync::{Arc, RwLock, Mutex}};
use super::utils::bitmap::Bitmap;
// What does our interface need?
// we must be able to 
// "get" a page
// "pin" a page during read/write
//
// ideally there is some form of 'ownership'
// i can apply such that all pages in use 
// are automatically "pinned"
//
// We must also implement some sort of cache eviction strategy
// ideally i should implement this using composition, 
// such that our pool contains a "eviction" strategy object that tracks usages

type ID = u32; 
pub struct Pool {
    // our buffer pool can use a map to track cached pages, and its frame in memory
    cache: Mutex<HashMap<ID, usize>>,
    frames: Vec<RwLock<Page>>,
    dirty: Mutex<Bitmap>,
    pinned: Vec<Mutex<u8>>,
    strategy: EvictionStrategy,
}

// is it unavoidable to use a trait for this interface?
pub enum EvictionStrategy {
    LruK,
    Clock,
}

pub struct Page {
    data: [u8; 4096],
}

impl Page {
    pub fn new() -> Self {
        return Page { data: [0; 4096] }
    }
}

pub struct PageGuard<'a> {
    data: &'a Pool,
    page_id: ID,
    pool_idx: usize,
}

impl Pool {
    // we need to init bitmaps, cache, and choose eviction strategy
    pub fn new(capacity: usize, strategy: EvictionStrategy) -> Self {
        let mut frames = Vec::new();
        let mut pinned = Vec::new();
        for _ in 0..capacity {
            frames.push(RwLock::new(Page::new()));
            pinned.push(Mutex::new(0 as u8));
        }
        Pool { 
            cache: Mutex::new(HashMap::new()),
            frames,
            dirty: Mutex::new(Bitmap::with_capacity(capacity)),
            pinned,
            strategy
        }
    }
    
    pub fn get_page(&self, page: ID) -> PageGuard{
        let idx = {
            let cache = self.cache.lock().unwrap();
            if cache.contains_key(&page) {
                *cache.get(&page).unwrap()
            }
            else {
                // TODO:
                // evict unpinned page,
                // remove that page from cache
                // read data from disk
                // move new page into cache
                // return correct idx
                1 as usize
            }
        };
        PageGuard::new(self, page, idx)
    }
}

impl<'a> PageGuard<'a> {
    pub fn new(pool: &'a Pool, page_id: ID, pool_idx: usize) -> Self {
        // have to get mutex before critical section
        let mut pin_count = pool.pinned[pool_idx].lock().unwrap();
        *pin_count += 1;
        // currently printing for debugging
        println!("picked up page {}, pin is {}", page_id, *pin_count);
        drop(pin_count);

        PageGuard { data: pool, page_id, pool_idx }
    }
}

impl<'a> Drop for PageGuard<'a> {
    fn drop(&mut self) {
        let idx = self.pool_idx;
        // acquire mutex
        let mut pin_count = self.data.pinned[idx].lock().unwrap();
        *pin_count -= 1;
        // currently printing for debugging
        println!("dropped page {}, pin is {}", self.page_id, *pin_count);
        drop(pin_count);
    }
}
