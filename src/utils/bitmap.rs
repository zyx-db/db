pub struct Bitmap{
    data: Vec<bool>
}

impl Bitmap {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0);
        Bitmap { data: Vec::with_capacity(capacity) }
    }

    pub fn set(&mut self, idx: usize){
        assert!(idx < self.data.len());
        self.data[idx] = true;
    }

    pub fn unset(&mut self, idx: usize){
        assert!(idx < self.data.len());
        self.data[idx] = false;
    }

    pub fn check(&self, idx: usize) -> bool {
        assert!(idx < self.data.len());
        self.data[idx]
    }
}
