//  Let's use file systems
use std::fs::File;

//  1: it looks like this
//  Result is used in a similar way as Option but to determine if we got a result or an error
//  It's a fined tuned enum used to manage error cases
//  Result is a reserved word, so in your programs: DON'T WRITE THAT it's just for the tutorial here
enum Result<T,E> {
    Ok(T),
    Err(E),
}


fn main() {
    // println!("Hello, world!");
    // panic!("killed myself tho");

    //  Let's see how to manage errors 

    //  Here how to open a file
    let working = File::open("hello.txt").unwrap(); // unwrap will skip the matching of Result
    //  But, if the file didn't exist, it would panic directly 
    println!("{:?}", working);

    //  comment this line to continue the program
    let error_and_panic = File::open("nope").expect("File not exists so panic");

    //  Let's see of to manage panic on our own
    let file = File::open("notfoundfile.txt");
    //  Let's overwrite the variable with the result
    let file = match file {
        Ok(data) => data,
        Err(e) => {
            println!("Nothing man");
            //  Let's panic and kill myself
            panic!("{:?}", e);
        }
    };

}

