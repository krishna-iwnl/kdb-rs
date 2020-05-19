use crate::defs;

struct Attribute{
    name : String,
    my_type: defs::Type,
}

struct Schema{
    num_atts: i32,
    my_atts: Vec<Attribute>,
    filename: String,
}