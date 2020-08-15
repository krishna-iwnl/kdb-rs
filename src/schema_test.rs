use crate::schema::Schema;
use crate::schema::Attribute;
use crate::defs;

#[test]
fn schema_success() {
    let table_filename = "orders.tbl";
    let sch = Schema::new("./data/test_catalog.sch", "orders");
    let atts = vec![
        Attribute{name: String::from("o_orderkey"), my_type: defs::Type::Int},
        Attribute{name: String::from("o_custkey"), my_type: defs::Type::Int},
        Attribute{name: String::from("o_orderstatus"), my_type: defs::Type::String},
        Attribute{name: String::from("o_totalprice"), my_type: defs::Type::Double},
        Attribute{name: String::from("o_orderdate"), my_type: defs::Type::String},
        Attribute{name: String::from("o_orderpriority"), my_type: defs::Type::String},
        Attribute{name: String::from("o_clerk"), my_type: defs::Type::String},
        Attribute{name: String::from("o_shippriority"), my_type: defs::Type::Int},
        Attribute{name: String::from("o_comment"), my_type: defs::Type::String},
    ];
    let expected_schema = Schema{
        num_atts: 9,
        my_atts: atts,
        table_filename: String::from(table_filename),
    };
    assert_eq!(sch, expected_schema);
}

#[test]
#[should_panic]
fn schema_no_table() {
    let _sch = Schema::new("./data/test_catalog.sch", "invalid_table_name");
}

#[test]
#[should_panic]
fn schema_no_file() {
    let _sch = Schema::new("./data/non-exsisting-file.sch", "orders");
}

#[test]
fn schema_find_att() {
    let sch = Schema::new("./data/test_catalog.sch", "orders");
    let att_idx = sch.find("o_shippriority");
    assert_eq!(att_idx.unwrap(), 7);
}

#[test]
fn schema_find_att_type() {
    let sch = Schema::new("./data/test_catalog.sch", "orders");
    let att_type = sch.find_type("o_shippriority");
    assert_eq!(att_type.unwrap(), &defs::Type::Int);
}