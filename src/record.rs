#[cfg(test)]
#[path = "./record_test.rs"]
pub mod record_test;
use crate::schema::Schema;
use crate::defs::TypeTrait;
use crate::defs::Type;
use crate::defs::IntType;
use crate::defs::FloatType;
use crate::defs::StringType;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct Record{
    cells: Vec<Box<dyn TypeTrait>>
}

impl Record{

    pub fn get_num_atts(&self) -> usize {
        self.cells.len()
    }

    pub fn consume(&mut self, comsume_me: Record) {
        self.cells = comsume_me.cells;
    }

    // pub fn copy(&mut self, copy_me: Record){
    //     self.cells = Vec::with_capacity(copy_me.get_num_atts());
    //     for cell in copy_me.cells.iter() {
    //     }
    // }

    pub fn project(_atts_to_keep: &[usize]){
        
    }

    pub fn suck_next_record(&mut self, my_schema: &Schema, reader: &mut BufReader<File>) -> Result<(), &str>{
        self.cells = Vec::new();
        let mut line = String::new();
        let num_bytes = reader.read_line(&mut line).expect("IO Error in suck_next_record");
        if num_bytes == 0 {
            return Err("EOF");
        }
        let mut splits: Vec<&str> = line.split('|').collect();
        splits.pop();
        let num_atts = my_schema.get_num_atts();
        let atts = my_schema.get_atts();
        if num_atts != splits.len() {
            return Err("Mismatch atts len for schema and record");
        }

        for i in  0..num_atts{
            match atts[i].my_type {
                Type::Int => {
                    let int_val: i32 = match splits[i].parse::<i32>(){
                        Ok(int) => int,
                        Err(why) => {
                            println!("cannot convert string to i32 in suck_next_record : {}", why);
                            return Err("i32 parsing error");
                        }
                    };
                    let int_cell = IntType {value: int_val};
                    self.cells.push(Box::new(int_cell));
                },
                Type::Float => {
                    let float_val: f64 = match splits[i].parse::<f64>(){
                        Ok(float) => float,
                        Err(why) => {
                            println!("cannot convert string to f64 in suck_next_record : {}", why);
                            return Err("f64 parsing error");
                        }
                    };
                    let float_cell = FloatType {value: float_val};
                    self.cells.push(Box::new(float_cell));
                },
                Type::String => {
                    let str_val = splits[i];
                    let string_cell = StringType {value: String::from(str_val)};
                    self.cells.push(Box::new(string_cell));
                }
            }
        }
        Ok(())
    }

    pub fn new() -> Record{
        Record{cells: Vec::new()}
    }

    pub fn print(&self) {
        for cell in self.cells.iter() {
            cell.print(); 
            print!("|");
        }
        println!("");
    }

    pub fn to_bytes() -> Vec<u8> {
        vec![33]
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for cell in self.cells.iter() {
            cell.print(); 
            let _res = write!(f, "|");
        }
        let _res = writeln!(f,"");
        Ok(())
    }
}