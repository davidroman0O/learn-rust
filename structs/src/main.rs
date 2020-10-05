
//  You need that directive to be able to print the whole struct 
#[derive(Debug)]
struct Human {
    first_name: String,
    age: u8,
}

impl Human {
    //  associated method (static)
    fn new(fname: String, a: u8) -> Human {
        Human { first_name: fname, age: a, }
    }

    //  by adding &self you specify a method of the instance
    fn say_hello(&self) {
        println!("Watashi wa {} desu", self.first_name);
    }

    fn nice_to_meet_you(&self) {
        println!("Dozou yoroshiku");
    }

}



fn main() {
    //  This is how we create an object, similar to Golang 
    let h1 = Human {
        first_name: String::from("David"),
        age: 30,
    };

    println!("{}", h1.first_name);
    println!("{:?}", h1);       //  inline
    println!("{:#?}", h1);      //  human readable
    
    
    let h2 = Human::new("Daniel".to_owned(), 42);

    println!("{}", h2.first_name);
    println!("{:?}", h2);       //  inline
    println!("{:#?}", h2);      //  human readable

    h1.say_hello();
    h1.nice_to_meet_you();
    h2.say_hello();
    h2.nice_to_meet_you();

}

