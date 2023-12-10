pub struct DiskManager {}

impl DiskManager {
    pub fn read(page_id: u32){
        eprintln!("read page {}", page_id)
    }

    pub fn write(page_id: u32){
        eprintln!("wrote page {} to disk", page_id)
    }
}
