


fn main() {

    let life = 42;

    //  Match
    match life {
        //  0 only
        0 => println!("death"),
        //  A range between 1 and 41 conditions
        1..=41 => println!("almost"), 
        //  We can use conditions, but we create a variable to represent "life"
        value if value < 0 => println!("yo wtf"),
        //  We can use _ to directly use "life"
        _ if life > 70 => println!("ok"),
        //  We can have a group  
        51 | 69 => println!("nice"),
        //  default:
        _ => println!("default"), // all conditions must have a coma 
    }

    //  Pattern binding 
    let (one, two) = (1, 2);
    println!("{} - {}", one, two);

    //  For no reason, you could use the _ to bypass a value
    let (three, _, four) = (3, 42, 4);
    println!("{} - {}", three, four);

    //  Rust is smart enough to match lookalike structures

    //  Plot twist, you can also specify one to be mutable
    let (no, mut yes) = (1, 0);
    yes = 12;
    println!("{} - {}", no, yes);

    //  And you can even have nested one
    let (funny, (_, mut nested), lol) = (2, (4, 13), 69);
    nested = 23;
    println!("{} - {} - {}", funny, nested, lol);

    //  Let' use tuples
    let tuple = ( "div", 42.0, 12.0);

    //  We have to respect the type of the variable all along
    match tuple {
        (op, x, y) if op == "div" => println!("draw {}", x / y),
        (op, x, y) if op == "add" => println!("add {} ", x + y),
        //  but if we don't need one var, we can bypass it with _
        (op, x, _) if op == "pow" => println!("pow {} ", x * x),
        _ => println!("unknown"),
    }

    //  But we can use the match to return a result also
    let (text, res) = match tuple {
        (op, x, y) if op == "div" => ("div", x / y),
        (op, x, y) if op == "add" => ("add", x + y),
        //  but if we don't need one var, we can bypass it with _
        (op, x, _) if op == "pow" => ("pow", x * x),
        _ => ("unknown", 0.0),
    };

    println!("{} {}", text, res);

    //  BUT you can return the tuple directly 
    let result = match tuple {
        (op, x, y) if op == "div" => ("div", x / y),
        (op, x, y) if op == "add" => ("add", x + y),
        //  but if we don't need one var, we can bypass it with _
        (op, x, _) if op == "pow" => ("pow", x * x),
        _ => ("unknown", 0.0),
    };
    //  But it will be really ugly code... 
    println!("{} {}", result.0, result.1);
}


