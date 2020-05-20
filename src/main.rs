pub mod defs;
pub mod schema;
pub mod config;

fn main() {
    println!("Hello, world!");
    let sch = schema::Schema::new(config::CATALOG_FILEPATH,"orders");
    println!("{:?}",sch);
}
