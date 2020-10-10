
#[derive(Debug)]
struct Rectangle {
    w: f32,
    h: f32,
}


impl Rectangle {
    fn surface(&self) -> f32 {
        (&self.w * &self.h)
    }
}

#[derive(Debug)]
struct Circle {
    r: f32,
}

impl Circle {
    fn surface(&self) -> f32 {
        (&self.r.powf(2.0) * 3.14f32)
    }
}

// fn compute<T>(shape: T) {
//     //  The compiler is not sure about all types having a method surface... so let's see traits
//     println!("{:?}", shape.surface());
// }

// So we can have an interface with a function who got the same signature
trait Compute {
    fn surface(&self) -> f32; // we can have code in it btw
    fn perimeter(&self) -> f32;
}

trait PrintMe {
    fn print(&self);
}

//  This is how we implement an interface for rectangle
impl Compute for Rectangle {
    fn surface(&self) -> f32 {
        (&self.w * &self.h)
    }
    fn perimeter(&self) -> f32 {
        (&self.w * &self.h) * 2.0f32
    }

}
impl Compute for Circle {
    fn surface(&self) -> f32 {
        (&self.r.powf(2.0) * 3.14f32)
    }
    fn perimeter(&self) -> f32 {
        (&self.r * 3.14f32) * 2.0f32
    }
}

impl PrintMe for Rectangle {
    fn print(&self) {
        println!("{:?}", &self);
    }
}

impl PrintMe for Circle {
    fn print(&self) {
        println!("{:?}", &self);
    }
}

//  Now we can have our function that receieve a trait
fn compute(shape: &impl Compute) -> f32 {
    println!("{:?}", shape.surface());
    shape.surface()
}

//  Here another syntax for generic
//  I personnaly perfer this one
fn gcompute<T: Compute>(shape: &T) -> f32 {
    println!("{:?}", shape.surface());
    shape.surface()
}


//  How to have multiple traits in one function ?
fn loud(shape: &(impl Compute + PrintMe)) {
    shape.surface();
    shape.print();
}
//  Same thing with generic 
fn gloud<T: Compute + PrintMe>(shape: &T) -> f32 {
    println!("{:?}", shape.surface());
    shape.surface()
}


//  How can we share the same method for different structs?
fn main() {
    
    let r = Rectangle{ w:12.0, h:12.0 };
    let c = Circle{ r: 3.14 };
    println!("{:?}", r.surface());
    println!("{:?}", c.surface());

    //  We got 2 different methods
    //  We should have a better way to implent the method surface on different types, right?
    //  What if we can to do process with a function with the surface function?
    //  see function compute<T>

    //  Let's see Traits (interface) now
    compute(&r); // & because we borrow the variable and not destroy it in this scope by passing it
    compute(&c);

    //  and now we can specify a generic type instead
    gcompute::<Rectangle>(&r);
    gcompute::<Circle>(&c);

    c.print();
    r.print();

    loud(&c);
    loud(&r);

    gloud::<Rectangle>(&r);
    gloud::<Circle>(&c);
}
