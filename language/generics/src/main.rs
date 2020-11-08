

#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32,
}

#[derive(Debug)]
//  By convention, generic T are un uppercase
struct GenericRectangle<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct DoubleRectangle<T, U>{
    x: T,
    y: U,
}

//  Implementation of a generic struct
impl<T> GenericRectangle<T> {
    //  Here how to create a simple getter
    fn get_x(&self) -> &T { // Since you remember the pointers, we return logically a pointer of the variable we return
        &self.x
    }

    //  T can be anything
    //  BUT since we do an operation on any type, it could be a string
    //  The compiler will refused to return a type &T
    //  
    // fn surface(&self) -> &T { 
        // &self.x * &self.y // We can't do that
    // }
}

//  For the surface method, since we can't implent for any types
//  We can make specialized implementation for each type
impl GenericRectangle<usize> {
    //  Specific for usize
    fn surface(&self) -> usize { 
        &self.x * &self.y // We can't do that
    }
}
impl GenericRectangle<f32> {
    //  Specific for usize
    fn surface(&self) -> f32 { 
        &self.x * &self.y // We can't do that
    }
}


fn main() {
    //  Remember structs?
    let rect = Rectangle{ x: 12, y: 12};
    println!("{:?}", rect);
    
    //  If we want to use f32 instead, we can't
    //  What if we want to have generic types intead of creating multiple structs for each structs
    //  Look at GenericRectangle
    let r = GenericRectangle::<f32>{ x: 32.23, y: 42.42};
    println!("{:?}", r); // so now it's working

    //  We can also skip the diamond operation
    let sr = GenericRectangle{x: "1", y:"2"}; // note: of course, we don't use &str but it's for the example
    println!("{:?}", sr); 
    
    //  But you can also have different types on one struct
    let dr = DoubleRectangle{x: "1", y: 32.2}; // note: of course, we don't use &str but it's for the example
    println!("{:?}", dr); 
    
    //  Now how to have methods for those generic structs ?
    println!("{:?}", r.get_x());

    let r = GenericRectangle::<usize>{ x: 32, y: 42};
    println!("{:?}", r.surface());
}
