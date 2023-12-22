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

impl BTreeNode {
    fn search(&self, key: TupleField) -> Option<Vec<TupleField>> {
        None
    }

    fn delete(&self, key: TupleField) {}

    fn insert(&self, key: TupleField, record: Vec<TupleField>) {}
}
