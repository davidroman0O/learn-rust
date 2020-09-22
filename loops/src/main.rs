

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
    let a = [42,2,3,4,5,6,7,8,9];
    for element in a.iter() {
        println!("{}", element);
    }
    // but if you need the index
    for (index, element) in a.iter().enumerate() {
        println!("{} - {}", index, element);
    }
    //  we can also use ranges
    for element in 1..6 {
        println!("range: {}", element);
    }

    //
    //  Labels
    //
    //  Let's write a double loop
    for one in 1..10 {
        for two in 1..10 {
            if two == 3 {
                // We break the loop "two" 
                break 
            }
            println!("{} {}", one, two);
        }
    }
    //  We can assign labels to loops
    'top: for one in 1..10 {
        'bottom: for two in 1..10 {
            if two == 3 {
                // We break the main loop directly 
                break 'top
            }
            if one == 9 {
                // We break the main loop directly 
                break 'bottom
            }
            println!("top {} bottom {}", one, two);
        }
    }


}
