use crate::bufferpool::Page;

use std::process::exit;
use std::sync::Mutex;
use std::io::{self, SeekFrom};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};

pub struct DiskManager {
    file: Mutex<File>,
}

impl DiskManager {
    pub fn new() -> Self {
        eprintln!("creating DiskManager");
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./files/db.dat");
        eprintln!("returning DiskManager");
        DiskManager { file: Mutex::new(f.unwrap()) }
    }

    pub fn read(&self, page_id: u32) -> Page {
        eprintln!("read called");
        let mut res: Page = Page::from([0; 4096]);
        let offset = SeekFrom::Start(4096 * page_id as u64);
        let mut file = self.file.lock().unwrap();
        file.seek(offset).unwrap();
        eprintln!("seeked success");
        file.read_exact(&mut res).unwrap();
        eprintln!("read page {}", page_id);
        res
    }

    pub fn write(&self, page_id: u32, page_content: &Page){
        let mut file = self.file.lock().unwrap();
        let offset = SeekFrom::Start(4096 * page_id as u64);
        file.seek(offset).unwrap();
        eprintln!("going to write");
        file.write(page_content).unwrap();
        eprintln!("wrote page {} to disk", page_id);
    }
}
