
//  
//  Language strongly typed
//  
fn main() {
    //  Type inference, compiler will automatically understand what type is is
    //  It's kinda a bad practice in rust
    //  ; is mandatory in this language 
    let variable = 42;
    
    //  println marco only accept strings - with templtes - an then your variables
    //  just like on Golang with fmt.Println 
    println!("variable: {}", variable);
    
    //  Use ":" to specify the type 
    //  unsigned (positive): u8 u16 u32 u64 u128
    let vunsigned: u8 = 42;
    println!("vunsigned: {}", vunsigned);
    
    //  signed: i8 i16 i32 i64 i128
    let vsigned: i8 = -42;
    println!("vsigned: {}", vsigned);
    
    //  isize (32bits on x86, 64bits on x64)
    let visize = -42;
    let vusize = 42;
    println!("visize: {}  || vusize: {}", visize, vusize);
    
    //  f32 f64
    let vfloat: f32 = 42.42;
    println!("vfloat: {}", vfloat);
    
    //  Use notation
    let vnotation = 42_000_000;
    println!("vnotation: {}", vnotation);
    //  hex
    let vhex = 0xff;
    println!("vhex: {}", vhex);
    //  binary
    let vbinary = 0b10011;
    println!("vbinary: {}", vbinary);

    //  Sufix for types
    let vsufix = 23u8;
    println!("vsufix: {}", vsufix);
    
    //  Mutability
    //  by default, all variable are immutable
    //  Note: immutable variable behave differently from constants, see later
    let mut changable: u16 = 23;
    changable = 42;
    println!("changable: {}", changable);

    //  Casting
    //  Note: you can't convert every types, only for compatible types
    let castme: u32 = 42.42424242424242f32 as u32;    
    println!("castme: {}", castme); // truncted decinals

    //  Constantes
    //  SCREAMING_CASE
    const CONSTANT_AH: f32 = 3.14;
    println!("CONSTANT_AH: {}", CONSTANT_AH);

    //  What's the difference?
    //  On the surface, no difference
    //  Behind the scene, "const" will be removed and hardcoded
    //  The binary will execute a code like this println!("CONSTANT_AH: {}", 3.14)
    //  It ensure better performances and less memory accesses
    //  It's a search and replace with extra steps
    //  That's why we need to know the type and value of this constant for the build

    //  Arithmetic
    let modulo = 89%12;
    println!("modulo: {}", modulo);

    //  Only if type works
    let float = 89f32 + 12.42;
    println!("float: {}", float);
    
    let boolean: bool = false;
    println!("boolean: {}", boolean);
    
    //  char are encoded in 32bit to have accents and emojis
    let character: char = 'k';
    println!("character: {}", character);
    
    //  strings
    let chain = "very short string";
    println!("chain: {}", chain);
    
    //  in reality, the type of "chain" is &str
    //  &str is immutable
    let rchain: &str = "very short string";
    println!("rchain: {}", rchain);

    let mut but_works_anyways: &str = "very short string";
    but_works_anyways = "change me";
    println!("but_works_anyways: {}", but_works_anyways);
    //  Because the compiler will see we wanted to change the value
    //  Behind the scene: the compiler will destroy the previous one and will re-create with a new variable
    //  we'll see later a more detailed explanation 
    
    //  str is primitive
    //  String is an object
    //  Here we use a static method from String "class" - we'll see why it's not a class -
    let big_string: String = String::from("very different string");
    println!("big_string: {}", big_string);
    
    //  It can be mutate also
    let mut mut_string: String = String::from("another one");
    mut_string = String::from("a better one");
    println!("mut_string: {}", mut_string);

    //  Let's use methods of String
    let mut string_methods: String = String::from("my string super cool");
    string_methods.push_str(" that's longer");
    println!("string_methods: {}", string_methods);
    
    //  Shadowing
    //  Tip: do not abuse it in your code
    let a: u16 = 256;
    let b = a.to_string();  //  The ide may not propose it 
    println!("b: {}", b);
    //  But you can't do that
    // let a: u16 = 256;
    // a = a.to_string();  //  The ide may not propose it 
    // println!("a: {}", a);
    //  Because u16 already got a type and you can't change it
    //  BUT you can use the shadowing
    let shad: u16 = 256;
    //  The compiler will destroy previous shad and create a new one with the same name
    //  shad: u16 doesn't exists after the new instanciation
    let shad = shad.to_string();
    println!("shad: {}", shad);
}
