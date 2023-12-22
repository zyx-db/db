use crate::page_interpretation::{TupleField, TupleFieldTypes};

pub struct BTree {
    root: u32,
}

pub enum BTreeNode {
    Leaf(BTreeLeaf),
    Internal(BTreeInternal),
}

pub struct BTreeLeaf {
    schema: Vec<([char; 256], TupleFieldTypes)>,
    data: Vec<Vec<TupleField>>,
}

pub struct BTreeInternal {
    schema: Vec<([char; 256], TupleFieldTypes)>,
    keys: Vec<TupleField>,
    pointers: Vec<u32>,
}

impl BTree {
    // fn new() -> Self {}

    pub fn search(&self, key: TupleField) -> Option<Vec<TupleField>> {
        let mut current_page = self.root;
        loop {
            // fetch current page from buffer pool
            // interpret it correctly, ex:
            // let BTreeNode = BTreeNode::from(page);

            // search within that node
            // handle cases:
            // !found -> return None
            // internal & found -> update current page
            // leaf & found -> return value

            return None;
        }
    }

    // try to delete with optimistic locking, otherwise pessimistic
    pub fn delete(&self, key: TupleField) {}

    fn optimistic_delete(&self, key: TupleField) -> bool {
        false
    }

    fn pessimistic_delete(&self, key: TupleField) {}

    // try to insert with optimistic locking, otherwise pessimistic
    pub fn insert(&self, key: TupleField, record: Vec<TupleField>) {}

    fn optimistic_insert(&self, key: TupleField, record: Vec<TupleField>) -> bool {
        false
    }

    fn pessimistic_insert(&self, key: TupleField, record: Vec<TupleField>) {}

    pub fn all_keys(&self) -> Option<Vec<TupleField>> {
        None
    }
}

impl BTreeNode {
    fn search(&self, key: TupleField) -> Option<Vec<TupleField>> {
        None
    }

    fn delete(&self, key: TupleField) {}

    fn insert(&self, key: TupleField, record: Vec<TupleField>) {}

    fn split(&self) {}

    fn all_keys(&self) -> Option<Vec<TupleField>> {
        None
    }
}
