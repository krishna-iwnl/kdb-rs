#[cfg(test)]
#[path = "./schema_test.rs"]
pub mod schema_test;
use std::io::prelude::*;

use std::io::{BufReader};
use std::fs::File;
use crate::defs;

#[derive(Debug,PartialEq)]
pub struct Attribute{
    name : String,
    pub my_type: defs::Type,
}

#[derive(Debug,PartialEq)]
pub struct Schema{
    num_atts: usize,
    my_atts: Vec<Attribute>,
    table_filename: String,
}

impl Schema{
    
    pub fn new(filename: &str, relname: &str) -> Schema{

        // Open the file
        let file = match File::open(filename) {
            Err(why) => panic!("couldn't open {}:" ,why.to_string()),
            Ok(file) => file,
        };

        let mut buf_reader = BufReader::new(file);
        let mut found = false;
        let mut catalog_vector : Vec<String> = Vec::new();
        loop {
            let mut buffer = String::new();
            match buf_reader.read_line(&mut buffer){
                Err(why) => {
                    println!("{} error in reading file",why);
                    break;
                },
                Ok(ok) => {
                    buffer.pop();
                    if found == true && buffer == "END".to_string() {
                        break;
                    }
                    else if buffer == relname.to_string() {
                        found = true;
                    }
                    else if found == true {
                        // println!("{}",buf);
                        catalog_vector.push(buffer);
                    }
                    else if ok==0 {
                        break;
                    }
                },
            }
        }

        if catalog_vector.len()==0 {
            panic!("Atts are empty!");
        }
        
        let mut table_filename:String=String::from("error");
        let mut my_atts: Vec<Attribute> = Vec::new();
        for (i,val) in catalog_vector.iter().enumerate(){
            if i==0{
               table_filename = String::from(val);
            }
            else{
                let att_type: Vec<&str> = val.split(' ').collect();
                let att :Attribute = match att_type[1] {
                    "String" => Attribute{name: att_type[0].to_string(),my_type: defs::Type::String},
                    "Int" => Attribute{name: att_type[0].to_string(),my_type: defs::Type::Int},
                    "Double" => Attribute{name: att_type[0].to_string(),my_type: defs::Type::Float},
                    _ => panic!("Wrong type in schema!"),
                };
                my_atts.push(att);
            }
        }
        Schema {
            num_atts: my_atts.len(),
            my_atts: my_atts,
            table_filename: table_filename,
        }
    }

    pub fn find(&self, attname: &str) -> Option<usize> {
        for (i,val) in self.my_atts.iter().enumerate() {
            if val.name == attname {
                return Some(i);
            }
        }
        None
    }
    
    pub fn find_type(&self, attname: &str) -> Option<&defs::Type> {
        let result :&defs::Type;
        for val in self.my_atts.iter() {
            if val.name == attname {
                result = &val.my_type;
                return Some(result); 
            }
        }
        None
    }

    pub fn get_num_atts(&self) -> usize {
        self.num_atts
    }

    pub fn get_atts(&self) -> &Vec<Attribute> {
        &self.my_atts
    }
    
}