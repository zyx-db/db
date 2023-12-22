use crate::bufferpool::Page;
use crate::utils::bitmap::Bitmap;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::sync::Mutex;

const SPECIAL_PAGES: u32 = 4;

fn merge_u8(first: u8, second: u8) -> u16 {
    (first as u16) << 8 | (second as u16)
}

fn split_u16(data: u16) -> (u8, u8) {
    let first: u8 = (data >> 8) as u8;
    let second: u8 = (data & 0xFF) as u8;
    (first, second)
}

fn get_file_offset(page_id: u32) -> SeekFrom {
    let physical_page = page_id + SPECIAL_PAGES;
    SeekFrom::Start(4096 * physical_page as u64)
}

fn min<T: Ord>(first: T, second: T) -> T {
    if first < second {
        first
    } else {
        second
    }
}

pub struct DiskManager {
    capacity: Mutex<u16>,
    used: Mutex<u16>,
    map: Mutex<Bitmap>,
    file: Mutex<File>,
}

impl DiskManager {
    pub fn new() -> Self {
        // open file
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./files/db.dat")
            .unwrap();

        // read first page of metadata
        let mut data = Page::from([0; 4096]);
        f.read_exact(&mut data).unwrap();

        // read first 2 pairs of ints as u16 for correct fields
        let capacity = merge_u8(data[0], data[1]);
        let used = merge_u8(data[2], data[3]);

        // read remaining page and populate bitmap
        let mut map = Bitmap::with_capacity(4092 * 8 + (4096 * 8 * 3));
        for i in 4..4096 {
            let cur = data[i];
            for bit in 0..8 {
                let map_offset = ((i - 4) * 8) + bit;
                if cur & (1 << bit) == (1 << bit) {
                    map.set(map_offset);
                }
            }
        }

        for page in 0..3 {
            f.read_exact(&mut data).unwrap();
            let initial_offset = 4092 * 8;
            let added_offset = (4096 * 8) * page;
            for i in 0..4096 {
                let cur = data[i];
                for bit in 0..8 {
                    let current_offset = (i * 8) + bit;
                    let map_offset = current_offset + initial_offset + added_offset;
                    if cur & (1 << bit) == (1 << bit) {
                        map.set(map_offset);
                    }
                }
            }
        }

        DiskManager {
            capacity: Mutex::new(capacity),
            used: Mutex::new(used),
            map: Mutex::new(map),
            file: Mutex::new(f),
        }
    }

    pub fn read(&self, page_id: u32) -> Page {
        let mut res: Page = Page::from([0; 4096]);
        let offset = get_file_offset(page_id);
        let mut file = self.file.lock().unwrap();
        file.seek(offset).unwrap();
        file.read_exact(&mut res).unwrap();
        eprintln!("read page {}", page_id);
        res
    }

    pub fn write(&self, page_id: u32, page_content: &Page) {
        let mut file = self.file.lock().unwrap();
        let offset = get_file_offset(page_id);
        file.seek(offset).unwrap();
        file.write(page_content).unwrap();
        eprintln!("wrote page {} to disk", page_id);
    }

    pub fn new_page(&self) -> Option<u32> {
        // find next empty page, just linear scan
        let mut map = self.map.lock().unwrap();
        let mut capacity = self.capacity.lock().unwrap();
        let mut used = self.used.lock().unwrap();

        // add new pages
        eprintln!("capacity: {}, used: {}", *capacity, *used);
        if *capacity == *used {
            if *capacity == 4096 * 8 {
                return None;
            }
            let new_capacity = min(*capacity + 64, 4096 * 8);
            let added_pages = new_capacity - *capacity;
            *capacity = new_capacity;

            let mut file = self.file.lock().unwrap();
            file.seek(SeekFrom::End(0)).unwrap();
            for _ in 0..added_pages {
                file.write(&[0 as u8; 4096]).unwrap();
            }
        }
        for i in 0..*capacity {
            if !map.check(i as usize) {
                map.set(i as usize);
                *used += 1;
                return Some(i as u32);
            }
        }
        // this path should never be ran, needed to compile
        None
    }

    pub fn delete_page(&self, page_id: u32) {
        let mut map = self.map.lock().unwrap();
        map.unset(page_id as usize);

        // TODO calculate which disk page and clear on disk
    }

    fn persist(&self) {
        let mut data = Page::from([0; 4096]);

        let capacity = self.capacity.lock().unwrap();
        let used = self.used.lock().unwrap();

        // serialize u16 fields
        let (cap1, cap2) = split_u16(*capacity);
        let (used1, used2) = split_u16(*used);

        drop(capacity);
        drop(used);

        data[0] = cap1;
        data[1] = cap2;
        data[2] = used1;
        data[3] = used2;

        let map = self.map.lock().unwrap();

        for i in 4..4096 {
            for bit in 0..8 {
                let cur = ((i - 4) * 8) + bit;
                if map.check(cur) {
                    data[i] |= 1 << bit;
                }
            }
        }
        drop(map);

        // persist changes to database
        let mut file = self.file.lock().unwrap();
        file.rewind().unwrap();
        file.write(&data).unwrap();
    }
}

impl Drop for DiskManager {
    fn drop(&mut self) {
        self.persist();
    }
}
