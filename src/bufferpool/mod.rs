pub mod eviction;
use std::{collections::HashMap, sync::{Arc, RwLock, Mutex, RwLockWriteGuard, MutexGuard}, usize};
use crate::disk::DiskManager;

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
    cache: RwLock<HashMap<ID, usize>>,
    frames: Vec<RwLock<Page>>,
    frame_to_id: Vec<Mutex<Option<ID>>>,
    dirty: Mutex<Bitmap>,
    pinned: Vec<Mutex<u8>>,
    strategy: Mutex<Box<dyn EvictionStrategy>>,
    disk: DiskManager
}

pub type Page = [u8; 4096];

pub struct PageGuard<'a> {
    data: &'a Pool,
    page_id: ID,
    pool_idx: usize,
}

pub trait EvictionStrategy {
    fn update_entry(&mut self, frame: usize);
    fn find_victim<'a>(&'a mut self, pool: &'a Pool) -> (RwLockWriteGuard<Page>, usize);
}

impl Pool {
    // we need to init bitmaps, cache, and choose eviction strategy
    pub fn new(capacity: usize, strategy: Mutex<Box<dyn EvictionStrategy>>) -> Self {
        let mut frames = Vec::with_capacity(capacity);
        let mut pinned = Vec::with_capacity(capacity);
        let mut frame_to_id = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            frames.push(RwLock::new([10; 4096]));
            pinned.push(Mutex::new(0 as u8));
            frame_to_id.push(Mutex::new(None));
        }
        Pool { 
            cache: RwLock::new(HashMap::new()),
            frames,
            dirty: Mutex::new(Bitmap::with_capacity(capacity)),
            frame_to_id,
            // free: Mutex::new(Bitmap::with_capacity(capacity)),
            pinned,
            strategy,
            disk: DiskManager::new()
        }
    }
    
    pub fn get_page(&self, page: ID) -> PageGuard{
        let idx = {
            let cache = self.cache.read().unwrap();
            if let Some(idx) = cache.get(&page) {
                *idx
            }
            else {
                drop(cache);
                let write_cache = self.cache.write().unwrap();
                if let Some(idx) = write_cache.get(&page){
                    *idx
                }
                else {
                    // eprintln!("couldnt find {} in {:?}", page, write_cache);
                    self.replace_entry(page, write_cache)
                }
            }
        };
        PageGuard::new(self, page, idx)
    }

    // returns what slot is now empty
    fn replace_entry(&self, new_page_id: ID, mut cache: RwLockWriteGuard<HashMap<u32, usize>>) -> usize {
        // we start by finding the page to remove, and acquire a write lock on it
        let mut strat = self.strategy.lock().unwrap();
        let (mut victim_guard, frame) = strat.find_victim(self);
        // remove old cached id
        let mut frame_to_id_guard = self.frame_to_id[frame].lock().unwrap();
        match *frame_to_id_guard {
            None => {}
            Some(victim_id) => {
                // eprintln!("set to remove {}", victim_id);
                cache.remove(&victim_id);
                
                // if frame is dirty -> flush changes
                // we do not need to do so if this frame did not store a page,
                // thats why its in this match clause
                let mut dirty_frames = self.dirty.lock().unwrap();
                if dirty_frames.check(frame){
                    let page_content = self.frames[frame].read().unwrap();
                    self.disk.write(victim_id,&(*&page_content));
                }
                dirty_frames.unset(frame);
            }
        }
        *frame_to_id_guard = Some(new_page_id);
        drop(frame_to_id_guard);
        // update the entry, removing old key and adding new one
        cache.insert(new_page_id, frame); 
        let new_frame = self.disk.read(new_page_id);
        // let new_frame = [0; 4096];
        *victim_guard = new_frame;

        // eprintln!("put id {} in frame {}, resulting in {:?}", new_page_id, frame, cache);
        drop(victim_guard);
        drop(cache);
        frame
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

impl Drop for Pool {
    fn drop(&mut self) {
        let dirty_frames = self.dirty.lock().unwrap();
        for i in 0..dirty_frames.len() {
            let frame_to_id = self.frame_to_id[i].lock().unwrap();
            if dirty_frames.check(i) && frame_to_id.is_some(){
                let page_content = self.frames[i].read().unwrap();
                self.disk.write(frame_to_id.unwrap(), &(*page_content));
            }
        }
    }
}

unsafe impl Send for Pool {}
unsafe impl Sync for Pool {}
