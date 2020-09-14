

fn main() {
    
    //  Classical while 
    let mut value = 0;
    while value < 10 {
        println!("{}", value);
        value += 1;
        if value % 2 == 0 {
            continue;
        }
        if value == 9 {
            break;
        }
    };

    //  Todo an infinite loop, you can use "loop"
    value = 0;
    loop {
        println!("{}", value);
        value += 1;
        if value == 9 {
            break;
        }
    }

    //  But you can return something like conditions
    value = 0;
    let result = loop {
        value += 1;
        if value == 5 {
            break value;
        }
    };
    println!("result {}", result);

    //
    //  For loop
    //
    

}
