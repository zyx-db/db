pub struct Bitmap{
    capacity: usize,
    data: Vec<u64>
}

impl Bitmap {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0);
        let size = {
            let mut s = capacity / 64;
            if capacity % 64 > 0 {
                s += 1;
            }
            s
        };
        Bitmap { capacity, data: vec![0; size] }
    }

    pub fn set(&mut self, idx: usize){
        assert!(idx < self.capacity);
        let offset = idx / 64;
        let bit = idx % 64;
        self.data[offset] |= 1 << bit;
    }

    pub fn unset(&mut self, idx: usize){
        assert!(idx < self.capacity);
        let offset = idx / 64;
        let bit = idx % 64;
        let mask = 1 << bit;
        let flipped = !mask;
        self.data[offset] &= flipped;
    }

    pub fn check(&self, idx: usize) -> bool {
        assert!(idx < self.capacity);
        let offset = idx / 64;
        let bit = idx % 64;
        let mask = 1 << bit;
        (self.data[offset] & mask) == mask
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;

    #[test]
    fn set() {
        let mut map = Bitmap::with_capacity(32);  
        assert_eq!(map.check(0), false);
        map.set(0);
        assert_eq!(map.check(0), true);
    }

    #[test]
    fn size_1() {
        let map = Bitmap::with_capacity(64);  
        assert_eq!(map.data.len(), 1 as usize);
    }
    
    #[test]
    fn size_big() {
        let map = Bitmap::with_capacity(256);
        assert_eq!(map.data.len(), 4 as usize);
    }

    #[test]
    fn size_awk() {
        let map = Bitmap::with_capacity(100);  
        assert_eq!(map.data.len(), 2 as usize);
    }

    #[test]
    fn false_positive() {
        let mut map = Bitmap::with_capacity(3);
        map.set(0);
        map.set(2);
        assert_eq!(map.check(1), false);
    }

    #[test]
    fn unset(){
        let mut map = Bitmap::with_capacity(1);
        map.set(0);
        map.unset(0);
        assert_eq!(map.check(0), false);
    }
}
