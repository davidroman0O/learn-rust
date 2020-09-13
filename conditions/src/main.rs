
fn main() {

    let life = 42;
    if life > 42 {
        println!("greater")
    } else if life == 42 {
        println!("LIFE")
    } else {
        println!("smaller")
    }

    //  we can also create a return from a condition
    let universe = if life == 42 {
        "magiiiiiic"
    } else {
        "looooseeeerrrr"
    };
    println!("{}", universe);

    //  but you can't test is "life" is not null
    //  null doesn't exists in the language
    //  This code won't work
    // let universe = if life {
    //     "magiiiiiic"
    // }
    // println!("{}", universe);
    
}
