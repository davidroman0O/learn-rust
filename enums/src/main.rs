use rand::prelude::*; // * will load everything under "prelude" library 


// 1
#[derive(Debug)]
enum States {
    None,
    Staged,
    Ready,
}

// 2: we can associate a type to an enum
#[derive(Debug)]
enum Food {
    Burger(String), // mean it will be a function with that type
    Pizza,
    None,
}

//  3: Let see Option without getting into the details of <T>
//  Since we don't have "null" in Rust, it use None
//  Or is got some value we'll match on Some
//  Fun fact: the <T> is called a diamond operator 
enum Option<T> { // Option is a reserved word, so in your programs: DON'T WRITE THAT it's just for the tutorial here
    Some(T),
    None, 
}

//  Let's use the previously seen random module
fn fetch_food() -> Option<Food> {
    match random::<u8>() {
        0...50 => Option::Some(Food::Burger("carnivore".to_owned())),
        51...150 => Option::Some(Food::Pizza),
        _ => Option::None,
    }
}

fn main() {
    //  1
    let status = States::None;
    println!("{:?}", status);

    match status {
        States::None => println!("none"),
        States::Staged => println!("staged"),
        States::Ready => println!("ready"),
    }

    // 2
    let fast_food = Food::Burger("hamgourgeois".to_owned());
    println!("{:?}", fast_food);

    match fast_food {
        Food::None => println!("no food for you"),
        Food::Pizza => println!("Just a pizza"),
        Food::Burger(x) => println!("Selected burger {}", x), // we can get the value of the enum like this
    }

    //  3: Let's see how it works
    //  You can execute multiple time an see a different result 
    match fetch_food() {
        Option::None => println!("nothing"),
        Option::Some(x) => println!("We got a {:?}", x),
    }
}


