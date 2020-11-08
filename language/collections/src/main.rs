
use std::collections::HashMap;


fn main() {

    //  an array of u8 with 4 elements
    let array_u8: [u8;4] = [ 4, 3, 2, 1 ];
    //  Note: if a string or number, println will be able to print it
    //  but for other types, we have to use {:?}, it's a debug operation for complex types
    println!("{:?}", array_u8);
    
    //  Let edit those arrays
    let mut mutate_array: [u8;4] = [ 4, 3, 2, 1 ];
    mutate_array[0] = 42;
    mutate_array[1] = mutate_array[1] * 2;
    println!("{:?}", mutate_array);
    //  Here how to know the lenght of an array
    println!("{:?}", mutate_array.len());

    //  Type inference works also
    let mut inference = [4];
    println!("{:?}", inference);
    
    //  You can also make matrixes
    let mut matrix = [[1,2], [3,4]];
    println!("{:?}", matrix);
    println!("{:?}", matrix[1][0]);
    
    //  Now let's talk about slices 
    let array = [1,2,3,4,5,6,7];
    //  Slices are pointer range of an array
    //  range is inclusive..exclusive 
    //  -   first element is incluse
    //  -   second element at this index will be excluded
    let slice = &array[0..3];
    //  Remember, those are a pointer reference from "array"
    println!("{:?}", slice);
    
    //  Of course, we can play with mut
    let mut mutateme = [1,2,3,4,5,6,7];
    let editme = &mut mutateme;
    //  Note: you can't edit "mutateme" since "editme" borrowed the pointer
    //  so you can't call "mutateme" again
    // mutateme[0] = 1989;
    editme[0] = 1989;
    //  NOTE: if you uncomment this line and execute
    //  you'll have an error, since you have using "editme" again after
    //  the pointer has been borrowed and since "editme" is called again, that mean
    //  you intent to have 2 pointers that's not possible
    //  but after it works because you're not using "editme" again afterward
    //  that mean the compiler will automatically destroy "editme" and allow "mutate" to continue
    // println!("{:?}", mutateme);
    println!("{:?}", editme);
    //  We don't use "editme" again later so the compiler will give back the pointer
    //  to "mutateme"
    println!("{:?}", mutateme);


    //
    //  Tuples
    //
    //  Tuples are likes array except you can have different types
    //  You can also let the type inference do it job
    let tuple_ex = ( 12i8, 42.23, "add");

    //  But you can also specify types
    let type_types: (u32, i32, f32, &str) = (69, -69, 69.69, "NOICE");
    println!("{}", type_types.3); 
    
    //  Nested types are allowed
    //  Look at this ugly boiiiiiii
    let type_boi = (42u32, 23f32, (4, 8), ("NICE", "MEMES"));
    //  use parentheses to speficy the index element and not the variable of this element
    println!("{}", (type_boi.3).1); //  It's cool but really unreadable man

    //
    //  Vectors
    //
    //  A vector can contain only one type
    let mut hector = vec![45, 34, 23, 45];
    hector.push(1111111);
    hector.remove(0);
    //  You will use frequently vecs because you don't need to specify the amount of
    //  element it have to store
    println!("{:?}", hector);
    //  We got 45
    println!("{:?}", hector[2]);
    //  You'll see we got a Some(45)
    //  Why? 
    println!("{:?}", hector.get(2));
    //  If we uncomment that, we'll have a runtime error because the index doesn't exists
    // println!("{:?}", hector[42]);
    //  But if we use get, we'll have None
    println!("{:?}", hector.get(42));
    //  So this method help you to know that no value exists at this index
    //  We can use a match to extract the data
    match hector.get(42) {
        None => println!("no value bro"),
        //  If it's Some(variable)
        Some(x) => {
            println!("It's {} ", x);
        },
    };

    //  We can also have slices with vectors
    let vec = vec![1,2,3,4,5,6];
    let slice = &vec[0..2];
    println!("{:?}", slice);

    //  Behind the scene, we may see that later, vec! is a macro 
    //  a macro wrap an implementation, we'll see later, just like println actually
    //  The original vec! is Vec
    //  Let's create a mutate Vec of u32
    let mut original: Vec<u32> = Vec::new();
    original.push(12);
    println!("original {:?}", original);

    //
    //  Hashmap
    //      we use an external package (aka crate) from use std::collections::hash_map
    //  k anv v are generic like Vec
    //  here we've use type inference for automatic detection from compiler
    let mut hash = HashMap::new();
    hash.insert("life", 42);
    hash.insert("loop", 23);
    println!("{:?}", hash);
    println!("{:?}", hash.len());
    println!("{:?}", hash.contains_key("loop"));


    //
    //  Strings are collection too
    //
    let vector_of_chars = String::from("my string is loooong");
    let immu = &vector_of_chars[0..5]; // became a slice of that string
    println!("{:?}", immu); // it's a &str, 
    //  So when you do that
    let slice_of_nothing = "I matter";
    //  It's still a &str because it's like the compiler created a string somewhere in memory
    //  But refer it as a pointer of a string, like a lice
}
