pub enum TupleFieldTypes {
    INT,
    SIGNEDINT,
    DOUBLE,
    CHAR,
    TEXT256,
    BOOL
}

pub enum TupleField {
    Int(u32),
    SignedInt(i32),
    Double(f32),
    Char(char),
    Text256([char;256]),
    Bool(bool)
}
