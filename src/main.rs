
#[allow(dead_code)]
fn vector_loop(vector: &[String]){
    for value in vector{
        println!("{}",value);
    }
}

#[allow(dead_code)]
extern crate chrono;
use chrono::{DateTime,Utc};
use std::time::{SystemTime,UNIX_EPOCH};


#[allow(dead_code)]
mod block;
use block::Block;
use block::BlockChain;
use block::current_time;
use block::block__chain;


mod person;
use person::Person;
fn main(){
    let mut person1 = block::Person::new();
    // person1.output();
    let mut person2 = block::Person{
        public_key: "1_public_key".to_string(),
        private_key: "1_private_key".to_string(),
        balance:    1000u64,
    };
    // person2.output();
    let mut person3 = block::Person{
        public_key: "2_public_key".to_string(),
        private_key: "2_private_key".to_string(),
        balance:    900u64,
    };
    // person3.output();

    let mut vector_person: Vec<block::Person> = Vec::new();
    vector_person.push(person1);
    vector_person.push(person2);
    vector_person.push(person3);
    
    let mut first_block = Block::genesis_block(&vector_person);
    first_block.output_data();

    let mut person4 = block::Person{
        public_key: "3_public_key".to_string(),
        private_key: "3_private_key".to_string(),
        balance:    800u64,
    };
    vector_person.push(person4);
    for value in &vector_person{
        value.output();
    }

}
