use crate::record::Record;
use crate::config;
use crate::consts;
use crate::schema::Schema;
use std::fs::OpenOptions;
use std::fs::File;

#[test]
fn suck_next_record_one() {
    // Arrange
    let lineitem_file = open_readonly(config::TESTING_LINEITEM_FILE);
    let mut reader = std::io::BufReader::new(lineitem_file);
    let lineitem_schema = Schema::new(config::TESTING_CATALOG_FILE, consts::LINEITEM);
    let mut rec = Record::new();

    // Act 
    let res = rec.suck_next_record(&lineitem_schema, &mut reader);

    // Assert
    assert_eq!(res.unwrap(), ());
    assert_eq!(rec.get_num_atts(), 16);
}

#[test]
fn suck_next_record_mismatch_schema_vs_rec_attcount() {
 
    // Arrange
    let lineitem_file = open_readonly(config::TESTING_LINEITEM_MISMATCH_ATTS);
    let mut reader = std::io::BufReader::new(lineitem_file);
    let lineitem_schema = Schema::new(config::TESTING_CATALOG_FILE, consts::LINEITEM);
    let mut rec = Record::new();

    // Act 
    let res = rec.suck_next_record(&lineitem_schema, &mut reader);

    // Assert
    assert_eq!(res.is_err(), true);
    assert_eq!(res, Err("Mismatch atts len for schema and record"));
}

#[test]
fn suck_next_record_i32_parse_error() {
    // Arrange
    let lineitem_file = open_readonly(config::TESTING_LINEITEM_INT_PARSE_ERROR);
    let mut reader = std::io::BufReader::new(lineitem_file);
    let lineitem_schema = Schema::new(config::TESTING_CATALOG_FILE, consts::LINEITEM);
    let mut rec = Record::new();

    // Act 
    let res = rec.suck_next_record(&lineitem_schema, &mut reader);

    // Assert
    assert_eq!(res.is_err(), true);
    assert_eq!(res, Err("i32 parsing error"));
}

#[test]
fn suck_next_record_f64_parse_error() {
    // Arrange
    let lineitem_file = open_readonly(config::TESTING_LINEITEM_FLOAT_PARSE_ERROR);
    let mut reader = std::io::BufReader::new(lineitem_file);
    let lineitem_schema = Schema::new(config::TESTING_CATALOG_FILE, consts::LINEITEM);
    let mut rec = Record::new();

    // Act 
    let res = rec.suck_next_record(&lineitem_schema, &mut reader);

    // Assert
    assert_eq!(res.is_err(), true);
    assert_eq!(res, Err("f64 parsing error"));
}

#[test]
fn consume_record() {
    let rec = get_dummy_record();
    assert_eq!(rec.get_num_atts(), 16);
    let mut rec2 = Record::new();
    // rec is moved by consuming
    rec2.consume(rec);
    assert_eq!(rec2.get_num_atts(), 16);
}

fn open_readonly(filename :&str) -> File {
    return OpenOptions::new()
    .read(true)
    .write(false)
    .open(filename)
    .unwrap();
}

fn get_dummy_record() -> Record {
    let lineitem_file = open_readonly(config::TESTING_LINEITEM_FILE);
    let mut reader = std::io::BufReader::new(lineitem_file);
    let lineitem_schema = Schema::new(config::TESTING_CATALOG_FILE, consts::LINEITEM);
    let mut rec = Record::new();
    let _ = rec.suck_next_record(&lineitem_schema, &mut reader);
    rec
}