pub enum TupleFieldTypes {
    INT,
    SIGNEDINT,
    DOUBLE,
    CHAR,
    BOOL
}

pub enum TupleField {
    Int(u32),
    SignedInt(i32),
    Double(f32),
    Char([char; 256]),
    Bool(bool)
}
