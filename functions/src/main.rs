

// But you can't override a new function "add"
fn add(a: i32, b: i32) -> i32 {
    //  You can use a return
    // return a + b;
    //  But the last line with ; is considered a return
    a + b
}

//  Add this won't works
//  But we'll see later how to do it in a different way
// fn add(a: f32, b: f32) -> f32 {
//     a + b
// }

fn greeting(name :String) {
    println!("{}", name);
}

fn greeting_working(name :String) -> String {
    println!("{}", name);
    return name;
}

fn greeting_ptr(name: &String)  {
    println!("{}", name);
}

//
fn addme(mut arg: i32) {
    arg += 10
}

// But to modify the value linked to the pointer, we need a mutable pointer
fn addme_ptr(arg: &mut i32) {
    // Since arg is a mutable pointer, we can't write on it
    // We need to specify we want to write on the VARIABLE LINKED TO THE POINTER
    // So you have to add a * to the pointer to express that you want to add 10 to the value linked to the pointer
    // That's called a dereference 
    *arg += 10
}

fn world(arg: &mut String) {
    // So why we don't have to put an *?
    // Why I don't need to dereference arg?
    // String is not a value, it's a smart pointer (see ownership part)
    // It's automatically dereferenced
    arg.push_str(" World");
}


// fn do_return() -> &u32 { // Here we got a "lifetime" issue
//     let value: u8 = 10;
//     // If we executed do_return(), it won't works since the variable "value" will be destroyed at the end of this scope
//     &value
//      //  We'll see later how to specify the lifetime
// }

fn main() {
    println!("add: {}", add(1, 3));

    //
    //  Ownership
    //

    //  Another behavior to help you understand the way variables works with ownership
    let firstname = String::from("David");
    greeting(firstname);
    // greeting(firstname); // I've comment this line, but you can uncomment to see the IDE telling you something wrong
    //  So why it is wrong to call the same function twice on the same variable?
    //  Imagine a variable like a coin you put in an arcarde machine
    //  The coin enter into a mecanism, used to help you to start the game
    //  But if the machine don't bring you back your coin, you'll never be able to use it
    //  It's the same thing with variable in Rust
    //  You gave the variable a function, it will use it
    //  If the function don't give back the variable, you can't use it, just like the coin you put in an arcarde machine
    //  This is for the ownership principe, for borrowing it's of course different and we'll see that right after
    let firstname = String::from("David");
    //  With a function that return the variable, we can use it back
    let firstname = greeting_working(firstname); // Remember we can declare multiple time the same name of variable since it's the same type
    let firstname = greeting_working(firstname); // the previous variable won't exists after behind delcare again
    println!("{}", firstname);

    //
    //  Borrowing
    //
    
    let firstname = String::from("Borrowing");
    greeting_ptr(&firstname);   //  Here we use a reference of the variable, on don't give the variable DIRECTLY, just BORROWED it 
    greeting_ptr(&firstname);
    // The variable stays in this scope, and don't really enter in the function
    // It's just the reference/pointer of the variable, not the variable on itself
    // It's like using a credit card for payment, you don't give you bank account and the cashier is picking whatever
    // you give a reference of that bank account and the machine will use the value to pick the needed value of it without taking you bank account 
    // NOTE: I don't know if this analogy is great xD

    //  We use mutability of course
    let mut mutateme = 5;
    addme(mutateme);
    println!("mutateme {}", mutateme); // you can see that mutateme didn't change at all
    //  Why? Because we pass variable by copy 
    //  So we have to pass a pointer... a mutable pointer!
    // Since we want to write on the value linked to the pointer adress
    addme_ptr(&mut mutateme);
    println!("mutateme {}", mutateme); // And it's working 

    // This concept of borrowing is important and a foundamental in Rust

    // What could happen with a String?
    let mut hello = "Hello".to_owned();
    world(&mut hello);
    println!("{}", hello); // And it's working 

    //
    //  Return pointers 
    //

    // Look at do_return

}