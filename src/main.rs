pub mod defs;
pub mod schema;
pub mod config;
pub mod record;

fn main() {
    println!("Hello, world!");
    let sch = schema::Schema::new(config::CATALOG_FILEPATH, "region");
    println!("{:?}",sch);
}
