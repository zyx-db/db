pub mod eviction;
use std::{collections::HashMap, sync::{Arc, RwLock, Mutex, RwLockWriteGuard}};
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
//
// TODO: create lock ordering to prevent silly mistakes 

type ID = u32; 
pub struct Pool {
    // our buffer pool can use a map to track cached pages, and its frame in memory
    cache: Mutex<HashMap<ID, usize>>,
    frames: Vec<RwLock<Page>>,
    dirty: Mutex<Bitmap>,
    free: Mutex<Bitmap>,
    pinned: Vec<Mutex<u8>>,
    strategy: Mutex<Box<dyn EvictionStrategy>>,
}

pub type Page = [u8; 4096];

pub struct PageGuard<'a> {
    data: &'a Pool,
    page_id: ID,
    pool_idx: usize,
}

pub trait EvictionStrategy {
    fn update_entry(&mut self, entry_id: ID);
    fn find_victim<'a>(&'a mut self, pool: &'a Pool) -> (RwLockWriteGuard<Page>, ID);
}

impl Pool {
    // we need to init bitmaps, cache, and choose eviction strategy
    pub fn new(capacity: usize, strategy: Mutex<Box<dyn EvictionStrategy>>) -> Self {
        let mut frames = Vec::new();
        let mut pinned = Vec::new();
        for _ in 0..capacity {
            frames.push(RwLock::new([0; 4096]));
            pinned.push(Mutex::new(0 as u8));
        }
        Pool { 
            cache: Mutex::new(HashMap::new()),
            frames,
            dirty: Mutex::new(Bitmap::with_capacity(capacity)),
            free: Mutex::new(Bitmap::with_capacity(capacity)),
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
                drop(cache);
                self.replace_entry(page)
            }
        };
        PageGuard::new(self, page, idx)
    }

    // TODO: WE DON'T CURRENTLY FLUSH DIRTY PAGE TO DISK
    // returns what slot is now empty
    fn replace_entry(&self, new_page_id: ID) -> usize {
        // we start by finding the page to remove, and acquire a write lock on it
        let mut strat = self.strategy.lock().unwrap();
        let (mut victim_guard, page_id) = strat.find_victim(self);
        // we also lock the cache, so we can modify it safely
        let mut cache = self.cache.lock().unwrap();
        // update the entry, removing old key and adding new one
        let idx = cache.remove(&page_id).unwrap();
        cache.insert(new_page_id, idx);
        
        // replace frame here
        // TODO: THIS IS WHERE WE MUST FLUSH CHANGES TO DISK
        // let new_frame: Page = FILE_IO();
        let new_frame = [4; 4096];
        *victim_guard = new_frame;

        drop(victim_guard);
        drop(cache);
        idx
    }
}

impl<'a> PageGuard<'a> {
    pub fn new(pool: &'a Pool, page_id: ID, pool_idx: usize) -> Self {
        // have to get mutex before critical section
        let mut pin_count = pool.pinned[pool_idx].lock().unwrap();
        *pin_count += 1;
        // currently printing for debugging
        // println!("picked up page {}, pin is {}", page_id, *pin_count);
        drop(pin_count);

        PageGuard { data: pool, page_id, pool_idx }
    }

    pub fn read(&self) -> Page {
        // return a clone of the data, after acquiring read permission
        let data = self.data.frames[self.pool_idx].read().unwrap();
        let res = data.clone();
        drop(data);

        return res;
    }

    pub fn write(&self) -> RwLockWriteGuard<Page> {
        // we need to acquire locks on the data and dirty bitmap
        // we update the data and drop these mutexes
        let mut dirty_map = self.data.dirty.lock().unwrap();
        let frame_data = self.data.frames[self.pool_idx].write().unwrap();
        dirty_map.set(self.pool_idx);
        drop(dirty_map);
        frame_data
    }
}

impl<'a> Drop for PageGuard<'a> {
    fn drop(&mut self) {
        let idx = self.pool_idx;
        // acquire mutex
        let mut pin_count = self.data.pinned[idx].lock().unwrap();
        *pin_count -= 1;
        // currently printing for debugging
        // println!("dropped page {}, pin is {}", self.page_id, *pin_count);
        drop(pin_count);
    }
}

unsafe impl Send for Pool {}
unsafe impl Sync for Pool {}
