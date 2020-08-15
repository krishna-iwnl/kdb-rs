pub const MAX_ANDS: i8 = 20;
pub const MAX_ORS: i8 = 20;
pub const PAGE_SIZE: i32 = 131072;

pub enum Target{
    Left,
    Right,
    Literal,
}

pub enum CompOperator{
    LessThan,
    GreaterThan,
    Equals,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type{
    Int,
    Double,
    String,
}