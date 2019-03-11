struct block__chain<'a,'b>{
    block_data: &'b Vec<Block<'a>>,
}
impl<'a,'b> block__chain<'a,'b>{
    fn output(&self){
        for value in self.block_data.iter(){
            value.output();
        }
    }
}
struct Block<'a>{
    data: &'a Vec<Person>,
}
impl<'a> Block<'a>{
    fn output(&self){
        for value in self.data.iter(){
            value.output();
        }
    }
}

struct Person{
    id:     i32,
    name:   String,
}
impl Person{
    fn output(&self){
        println!("id:   {}",&self.id);
        println!("name: {}",&self.name);
        println!("----------");
    }
}

fn main(){

    let mut person1 = Person{
        id:     1,
        name:   "a".to_string(),
    };
    let mut person2 = Person{
        id:     2,
        name:   "b".to_string(),
    };
    let mut vector_person: Vec<Person> = Vec::new();
    vector_person.push(person1);
    vector_person.push(person2);

    {
    let mut block = Block{  
        data: &vector_person,
    };
    // block.output();
    }
    let mut person3 = Person{
        id:     3,
        name:   "c".to_string(),
    };
    // let mut vector_person.push(person3);
    // let mut vector_person: Vec<Person> = Vec::new();
    // mut vector_person: Vec<Person> = Vec::new();
    vector_person.push(person3);
    for value in &vector_person{
        // println!("{}",value);
        value.output();
    }

}