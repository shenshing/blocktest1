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
        println!("Public_Key:   {}",format!("*****{}*****",&self.public_key));
        println!("Private_Key:   {}",format!("*****{}*****",&self.private_key));
        println!("Balance:      {}",format!("#####${}#####",&self.balance));
        println!("------------------------------\n");
    }
}

pub type Human = Vec<Person>;
pub fn human_to_str(human:  &Human) -> String{
    let mut totalString = format!("");
    let mut totalString1 = format!("");
    for value in human{
        totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
        totalString = totalString + &totalString1;
    }
    return totalString
}


pub struct Testing{
    data: Vec<i32>,
}
impl Testing{
    fn new_instance(vect: Vec<i32>) -> Testing{
        Testing{
            data:    vect,
        }
    }
    fn output(&self){
        for v in self.data.iter(){
            println!("{}",v);
        }
    }
}



fn main(){

    let mut vector: Vec<i32> = Vec::new();
    let value1 = 1;
    let value2 = 2;
    let value3 = 3;
    vector.push(value1);
    vector.push(value2);
    vector.push(value3);
    let mut testing = Testing::new_instance(vector.clone());
    testing.output();
    println!("------------------");
    
    let mut value4 =  10;
    vector.push(value4);
    testing = Testing::new_instance(vector.clone());
    testing.output();

}

pub fn vector_(vector: &Vec<Person>){
    for value in vector{
        value.output();
    }
}