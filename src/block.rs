// extern crate chrono;
// use chrono::{DateTime,Utc};
// pub type Hash = Vec<u8>;
// pub type Human = Vec<Person>;

// struct block__chain{
//     block_data: Vec<Block>,
// }

// pub struct Person{
//     public_key:     String,
//     private_key:    String,
//     balance:        u64,
// }
// impl Person{
//     pub fn new() -> Self{
//         Person{ 
//             public_key:     "none_public_key".to_string(),
//             private_key:    "none_private_key".to_string(),
//             balance:        0u64,
//         }
//     }
//     pub fn new_instance(public_key: String,private_key: String,balance: u64) -> Person{
//         Person{
//             public_key,
//             private_key,
//             balance,
//         }
//     }
//     pub fn output(&self){
//         println!("\tPublic_Key:   {}",format!("*****{}*****",&self.public_key));
//         println!("\tPrivate_Key:   {}",format!("*****{}*****",&self.private_key));
//         println!("\tBalance:      {}",format!("#####${}#####",&self.balance));
//         println!("\t------------------------------\n");
//     }
// }



// impl block__chain{
//     pub fn new() ->  Self{
//         block__chain{
//             block_data: vec![],
//         }
//     }
//     pub fn push_block(&mut self,new_block: Block){
//         &self.block_data.push(new_block);
//     }
//     pub fn show_block(&self){
//         for value in &self.block_data{
//            Block::output_block(value); 
//         }
//     }
// }
//     use std::io;
//     pub trait BlockChain{
//         fn genesis_block(human: Human) -> Block;
//         fn block_instance(&self,human: Human) -> Block;
//         fn mine_block(&self,block: &[Block]);
//         fn input_data(&mut self);
//         fn output_data(&self);
//         fn output_block(block: &Block);
//         fn show_chain(&self,blockchain: &[Block]);
//         fn combine_string(&self) -> String;
//         fn checking_hash(blocks: &Vec<Block>) -> bool;
//     }
//     pub struct Block{
//         id:         u32,
//         timestamp:  String,
//         hash: Hash,
//         pub prev_hash:  Hash,
//         data:       Human,
//     }

//     impl BlockChain for Block{
//         fn genesis_block(human: Human) -> Block{
//             let time = current_time();
//             let mut value_hash: Vec<u8> = Vec::new();
//             let mut st1 = human_to_str(&human);
//             st1 = st1 + &time;
//             value_hash = hash(&st1);
//             Block{
//                 id: 0u32,
//                 timestamp: time,
//                 hash: value_hash, 
//                 prev_hash: vec![0;32],
//                 data: human,
//             }
//         }


//         fn block_instance(&self,human: Human) -> Block{
            
//             let time = current_time();
//             let mut value_hash: Vec<u8> = Vec::new();
//             let mut st1 = human_to_str(&human);
//             st1 = st1 + &time;
//             value_hash = hash(&st1);
//             Block{
//                 id: &self.id + 1,
//                 timestamp: time,
//                 hash: value_hash,
//                 prev_hash: self.hash.clone(),
//                 data: human,
//             }
//         }

//         fn mine_block(&self, block: &[Block]){
//             let mut data = String::new();
//             io::stdin().read_line(&mut data).expect("Invalid input");
//         }

//         fn input_data(&mut self){
//             let mut id = String::new();
//             println!("input id: ");
//             io::stdin().read_line(&mut id).expect("Something went wrong!");
//             self.id = id.trim().parse().unwrap();
//         }
//         fn output_data(&self){
//             println!("ID:           {}",&self.id);
//             println!("Timestamp:    {}",&self.timestamp);
//             println!("Prev_hash:    {}",vec_to_str(&self.prev_hash));
//             println!("Hash:         {}",vec_to_str(&self.hash));
//             println!("Data: ->");
//             output_person(&self.data);
//         }
//         fn output_block(block: &Block){
//             println!("ID:           {}",block.id);
//             println!("Timestamp:    {}",block.timestamp);
//             println!("Prev_hash:    {}",vec_to_str(&block.prev_hash));
//             println!("Hash:         {}",vec_to_str(&block.hash));
//             println!("Data: ->");
//             output_person(&block.data);
//         }
//         fn show_chain(&self,blockchain: &[Block]){
//             for block in blockchain{
//                 Block::output_block(&block);
//                 println!("----------*----------");
//             }
//         }

//         fn combine_string(&self) -> String{
//             let mut st = String::new();
//             st = st + &self.id.to_string() + &self.timestamp + &vec_to_str(&self.hash) + &vec_to_str(&self.prev_hash) /*+ vec_to_str1(&self.data)*/;
//             st
//         }
//         fn checking_hash(blocks: &Vec<Block>) -> bool{
//             true
//         }
        
//     }

// pub fn current_time() -> String{
//         let now: DateTime<Utc> = Utc::now();
//         let mut time = String::new();
//         time = now.format("%a %e %b %Y %T.%f").to_string();
//         time
// }

// pub fn hash (data: &String) -> Hash {
//             crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
//         }

// pub fn vec_to_str(vector: &Vec<u8>) -> String{
//     let mut st = String::new();
//     st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
//     st
// }

// pub fn human_to_str(human:  &Human) -> String{
//     let mut totalString = format!("");
//     let mut totalString1 = format!("");
// pub fn human_to_str(human:  &Human) -> String{
//     let mut totalString = format!("");
//     let mut totalString1 = format!("");
//     for value in human{
//         totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
//         totalString = totalString + &totalString1;
//     }
//     return totalString
// }
//     for value in human{
//         totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
//         totalString = totalString + &totalString1;
//     }
//     return totalString
// }

// pub fn output_person(vector: &Vec<Person>){
//     for value in vector.iter(){
//         value.output();
//     }
// }

// fn st(s: &String) -> String{
//     s.to_string()
// }

// fn main(){
    
// }
extern crate chrono;
use chrono::{DateTime,Utc};
pub type Hash = Vec<u8>;
pub type Human = Vec<Person>;

pub struct block__chain<'a,'b>{
    block_data: &'b mut Vec<Block<'a>>,
}

pub struct Person{
    pub public_key:     String,
    pub private_key:    String,
    pub balance:        u64,
}
impl Person{
    pub fn new() -> Self{
        Person{ 
            public_key:     "none_public_key".to_string(),
            private_key:    "none_private_key".to_string(),
            balance:        0u64,
        }
    }
    pub fn new_instance(public_key: String,private_key: String,balance: u64) -> Person{
        Person{
            public_key,
            private_key,
            balance,
        }
    }
    pub fn output(&self){
        println!("\tPublic_Key:   {}",format!("*****{}*****",&self.public_key));
        println!("\tPrivate_Key:   {}",format!("*****{}*****",&self.private_key));
        println!("\tBalance:      {}",format!("#####${}#####",&self.balance));
        println!("\t------------------------------\n");
    }
}



impl<'a,'b> block__chain<'a,'b>{
    pub fn push_block(&mut self,new_block: Block<'a>){
        &self.block_data.push(new_block);
    }
    pub fn show_block(&self){
        for value in self.block_data.iter(){
           Block::output_block(value); 
        }
    }
}
    use std::io;
    pub trait BlockChain<'a>{
        fn genesis_block(human: &Human) -> Block;
        fn block_instance(&self,human: &'a Human) -> Block;
        fn mine_block(&self,block: &[Block]);
        fn input_data(&mut self);
        fn output_data(&self);
        fn output_block(block: &Block);
        fn show_chain(&self,blockchain: &[Block]);
        fn combine_string(&self) -> String;
        fn checking_hash(blocks: &Vec<Block>) -> bool;
    }
    pub struct Block<'a>{
        id:         u32,
        timestamp:  String,
        hash: Hash,
        pub prev_hash:  Hash,
        data:       &'a Human,
    }

    impl<'a> BlockChain<'a> for Block<'a>{
        // fn new() -> Self{
        //     Block{
        //         id: 0u32,
        //         timestamp: "null".to_string(),
        //         hash:      "null".to_string(),
        //         prev_hash:  "null".to_string(),
        //         data:   
        //     }
        // }
        fn genesis_block(human: &Human) -> Block{
            let time = current_time();
            let mut value_hash: Vec<u8> = Vec::new();
            let mut st1 = human_to_str(&human);
            st1 = st1 + &time;
            value_hash = hash(&st1);
            Block{
                id: 0u32,
                timestamp: time,
                hash: value_hash, 
                prev_hash: vec![0;32],
                data: human,
            }
        }


        fn block_instance(&self,human: &'a Human) -> Block{
            
            let time = current_time();
            let mut value_hash: Vec<u8> = Vec::new();
            let mut st1 = human_to_str(&human);
            st1 = st1 + &time;
            value_hash = hash(&st1);
            Block{
                id: &self.id + 1,
                timestamp: time,
                hash: value_hash,
                prev_hash: self.hash.clone(),
                data: human,
            }
        }

        fn mine_block(&self, block: &[Block]){
            let mut data = String::new();
            io::stdin().read_line(&mut data).expect("Invalid input");
        }

        fn input_data(&mut self){
            let mut id = String::new();
            println!("input id: ");
            io::stdin().read_line(&mut id).expect("Something went wrong!");
            self.id = id.trim().parse().unwrap();
        }
        fn output_data(&self){
            println!("ID:           {}",&self.id);
            println!("Timestamp:    {}",&self.timestamp);
            println!("Prev_hash:    {}",vec_to_str(&self.prev_hash));
            println!("Hash:         {}",vec_to_str(&self.hash));
            println!("Data: ->");
            output_person(&self.data);
        }
        fn output_block(block: &Block){
            println!("ID:           {}",block.id);
            println!("Timestamp:    {}",block.timestamp);
            println!("Prev_hash:    {}",vec_to_str(&block.prev_hash));
            println!("Hash:         {}",vec_to_str(&block.hash));
            println!("Data: ->");
            output_person(&block.data);
        }
        fn show_chain(&self,blockchain: &[Block]){
            for block in blockchain{
                Block::output_block(&block);
                println!("----------*----------");
            }
        }

        fn combine_string(&self) -> String{
            let mut st = String::new();
            st = st + &self.id.to_string() + &self.timestamp + &vec_to_str(&self.hash) + &vec_to_str(&self.prev_hash) /*+ vec_to_str1(&self.data)*/;
            st
        }
        fn checking_hash(blocks: &Vec<Block>) -> bool{
            true
        }
        
    }

pub fn current_time() -> String{
        let now: DateTime<Utc> = Utc::now();
        let mut time = String::new();
        time = now.format("%a %e %b %Y %T.%f").to_string();
        time
}

pub fn hash (data: &String) -> Hash {
            crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
        }

pub fn vec_to_str(vector: &Vec<u8>) -> String{
    let mut st = String::new();
    st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    st
}

pub fn human_to_str(human:  &Human) -> String{
    let mut totalString = format!("");
    let mut totalString1 = format!("");
pub fn human_to_str(human:  &Human) -> String{
    let mut totalString = format!("");
    let mut totalString1 = format!("");
    for value in human{
        totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
        totalString = totalString + &totalString1;
    }
    return totalString
}
    for value in human{
        totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
        totalString = totalString + &totalString1;
    }
    return totalString
}

pub fn output_person(vector: &Vec<Person>){
    for value in vector.iter(){
        value.output();
    }
}

fn st(s: &String) -> String{
    s.to_string()
}

fn main(){
    
}