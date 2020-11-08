
//  This concept is important
fn main() {

    let life: u32 = 42;
    let mut orwell = life;
    orwell = 1989;
    println!("{} - {}", life, orwell);
    //  We see that orwell only was affected and not life

    //  Let's do the same thing but... with strings
    //  Issues with strings 
    //  THIS CODE WON'T RUN
    // let life_string = "something".to_owned(); // SAME THING as String::from, just to show example
    // let mut orwell_string = life_string;
    // println!("{} - {}", life_string, orwell_string);
    
    //  And we got an error:
    // println!("{} - {}", life_string, orwell_string);
    //    |                ^^^^^^^^^^^ value borrowed here after move
    
    //  Let's do the same thing, but diffrently
    let life_string = "something".to_owned(); 
    let orwell_string = life_string; // not mut
    println!("{} ", orwell_string); // only one variable
    // And it's working !
    // Why? It's a macanism called ownership

    /*
        You know this simple case
        variable1 ----> value1
        variable2 ----> value2

        Here we did:
        variable1 ----> &pointer1 ---> value1
        variable2 = variable1 means:
        variable2 ----> &pointer2 ---> value1
        
        That mean we COULD have two pointer to one value... BUT
        The compiler will automatically "destroy" variable1 (so pointer1) to only have ONE UNIQUE POINTER ALL THE TIME

        It's like moving &pointer1 to variable2, then destroying variable1
        So you can't use variable1 anymore!!

        That's called ownership, a variable own a pointer and give it to another variable and die
    */

    //  But we can clone the value, so a new pointer 
    let life_string_2 = "something".to_owned(); 
    let mut orwell_string_2 = life_string_2.clone();
    orwell_string_2.push_str(" longer");
    println!("{} - {}", life_string_2, orwell_string_2); // only one variable

    //  Scope
    let vscope = "scope".to_owned();
    //  create a bloc
    {
        let scoped = vscope;
        println!("{}", scoped);
        // scoped destroyed at the end of this bloc
    }
    //  You can't print scopped here cause the variable only exist in the previous block scope
    // println!("{}", scoped);
    //  You also can't print vscope anymore since the pointer was destroy in the last bloc when passed to "scoped"
    // println!("{}", vscope);
}

