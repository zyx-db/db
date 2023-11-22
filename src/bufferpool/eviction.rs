use super::{EvictionStrategy, Pool};
use std::collections::{HashSet, BinaryHeap};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::RwLockWriteGuard;

type Page = [u8; 4096];
type ID = u32;

#[derive(Clone)]
struct TimeRingBuffer {
    page_id: ID,
    head: usize,
    times: Vec<u128>,
}

pub struct LruK {
    heap: BinaryHeap<TimeRingBuffer>,
    current_pages: HashSet<ID>,
    buffer_size: usize,
    k: usize,
}

impl EvictionStrategy for LruK {
    fn update_entry(&mut self, entry_id: super::ID) {
        let current_time = SystemTime::now();
        // Calculate the duration since the Unix epoch
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        // Convert the duration to milliseconds
        let milliseconds_since_epoch = duration_since_epoch.as_millis();

        // we have the this page, lets find the entry
        // from there we can create a new, updated entry
        // we have to remove old entry, and insert new one
        if self.current_pages.contains(&entry_id){
            let copy = self.heap
                .iter()
                .find(|x| x.page_id == entry_id)
                .map(|s| s.clone())
                .unwrap();

            let new_entry = copy.update(milliseconds_since_epoch);
            self.heap.retain(|x| x.page_id != entry_id);
            self.heap.push(new_entry);
        }
        // otherwise we don't have this page,
        // we must create an entry and add it
        else {
            let new_entry = TimeRingBuffer::from(entry_id, self.k, milliseconds_since_epoch);
            self.heap.push(new_entry);
        }

        self.current_pages.insert(entry_id);
    }

    fn find_victim<'a>(&'a mut self, pool: &'a Pool) -> (RwLockWriteGuard<super::Page>, ID) {
        let buffer = self.heap.pop().unwrap();
        let buffer_id = buffer.page_id;

        let page_idx_mapping = pool.cache.lock().unwrap();
        let idx = page_idx_mapping.get(&buffer_id).unwrap().clone();
        drop(page_idx_mapping);

        self.current_pages.remove(&buffer_id);
        (pool.frames[idx].write().unwrap(), buffer_id)
    }
}

impl LruK {
    pub fn new(buffer_size: usize, k: usize) -> Self {
        let mut heap = BinaryHeap::new();
        for _ in 0..buffer_size {
            let b = TimeRingBuffer::new(0, k);
            heap.push(b);
        }
        LruK { heap, current_pages: HashSet::new(), buffer_size, k }
    }
}

// ordering for this is flipped compared to normal
// this is because i am using a max heap, but we want the oldest value
// the oldest value will have the smallest time
impl Ord for TimeRingBuffer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..self.times.len() {
            let first = self.times[i];
            let second = other.times[i];
            if first < second {
                return std::cmp::Ordering::Greater;
            }
            if second > first {
                return std::cmp::Ordering::Less;
            }
        } 

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for TimeRingBuffer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimeRingBuffer {
    fn eq(&self, other: &Self) -> bool {
        let size = self.times.len();
        for offset in 1..size+1 {
            let i = (self.head + offset) % size;
            let j = (other.head + offset) % size;
            if self.times[i] != other.times[j] {
                return false;
            }
        }
        return true;
    }
}

impl Eq for TimeRingBuffer {}

impl TimeRingBuffer {
    fn new(page_id: ID, size: usize) -> Self{
        TimeRingBuffer { page_id, head: 0, times: vec![0 as u128; size] }
    }

    fn from(page_id: ID, size: usize, time: u128) -> Self {
        let mut times = vec![0 as u128; size];
        times[0] = time;
        TimeRingBuffer { page_id, head: 0, times }
    }

    fn update(&self, time: u128) -> Self{
        let mut updated = self.clone();
        updated.head = (updated.head + 1) % updated.times.len();
        updated.times[updated.head] = time;
        updated
    }
}
