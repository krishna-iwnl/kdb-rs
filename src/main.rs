pub mod defs;
pub mod schema;
pub mod config;
pub mod record;
pub mod consts;

use std::fs::OpenOptions;

fn main() {

    let readfile = OpenOptions::new()
    .read(true)
    .write(false)
    .open(config::LINEITEM_10MB)
    .unwrap();

    // match write!(&mut file, "{}",s){
    //     Ok(_ok) => println!("ok"),
    //     Err(err) => println!("Err {}",err)
    // }
    // let b = s.as_bytes();
    // let i :i32 = 32;
    // let ii = i.to_be_bytes();

    // let mut a: Vec<u8> = Vec::new();

    // a.extend_from_slice(b);
    // println!("{:?}",b);
    // println!("{:?}",ii);
    // let v = vec!["1","2","3"];
    // let s = &v[0..1];
    // print!("{:?}",s);
    // file.write(&ii);

    // let mut read_buf = [0;11];
    // readfile.read(&mut read_buf);
    // println!("{:?}",read_buf);
    // a[5] = 4;
    // println!("{}", a[5]);
    let mut reader = std::io::BufReader::new(readfile);
    let my_schema = schema::Schema::new(config::CATALOG_FILEPATH, "lineitem");
    let mut rec = record::Record::new();

    // let i:usize = 0;
    // let ii:i32 = 3;
    // println!("{:?}", i.to_be_bytes());
    // println!("{:?}", ii.to_be_bytes());
    let _ = rec.suck_next_record(&my_schema, &mut reader);
    rec.print();
    let mut rec2 = record::Record::new();
    rec2.consume(rec);
    rec2.print();
    // loop {
    //     match rec.suck_next_record(&my_schema, &mut reader){
    //        Ok(()) => (),
    //        Err(why) => {
    //            println!("ERR {}",why);
    //            break;
    //        }
    //     }
    //     println!("{}", rec);
    //     // rec.print();
    // }
    
}
