use super::{EvictionStrategy, Pool};
use std::collections::BinaryHeap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::RwLockWriteGuard;


#[derive(Clone, Debug)]
struct TimeRingBuffer {
    frame: usize,
    head: usize,
    times: Vec<u128>,
}

pub struct LruK {
    heap: BinaryHeap<TimeRingBuffer>,
}

impl EvictionStrategy for LruK {
    fn update_entry(&mut self, frame: usize) {
        let current_time = SystemTime::now();
        // Calculate the duration since the Unix epoch
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        // Convert the duration to milliseconds
        let milliseconds_since_epoch = duration_since_epoch.as_millis();

        // we have the this frame, lets find the entry
        // from there we can create a new, updated entry
        // we have to remove old entry, and insert new one
        let copy = self.heap
            .iter()
            .find(|x| x.frame == frame)
            .map(|s| s.clone())
            .unwrap();

        let new_entry = copy.update(milliseconds_since_epoch);
        self.heap.retain(|x| x.frame != frame);
        self.heap.push(new_entry);
    }

    fn find_victim<'a>(&'a mut self, pool: &'a Pool) -> (RwLockWriteGuard<super::Page>, usize) {
        let buffer = self.heap.pop().unwrap();
        let frame_idx = buffer.frame;

        (pool.frames[frame_idx].write().unwrap(), frame_idx)
    }
}

impl LruK {
    pub fn new(buffer_size: usize, k: usize) -> Self {
        let mut heap = BinaryHeap::new();
        for i in 0..buffer_size {
            let b = TimeRingBuffer::new(i, k);
            heap.push(b);
        }
        LruK { heap }
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
    fn new(frame: usize, size: usize) -> Self{
        TimeRingBuffer { frame, head: 0, times: vec![0 as u128; size] }
    }

    fn from(frame: usize, size: usize, time: u128) -> Self {
        let mut times = vec![0 as u128; size];
        times[0] = time;
        TimeRingBuffer { frame, head: 0, times }
    }

    fn update(&self, time: u128) -> Self{
        let mut updated = self.clone();
        updated.head = (updated.head + 1) % updated.times.len();
        updated.times[updated.head] = time;
        updated
    }
}
