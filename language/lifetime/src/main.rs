
//  Not really tricky subject but we need to understand how it works
fn main() {
    let a = 12.12;
    let b = 13.12;
    //  We pass pointer cause we want to keep our a and b alive in this scope
    println!("{:?}", gt(&a, &b));
}

//  In order to return the greater variable, you already know we need to use pointers to keep the parent scope with the variable (borrowing)
//  BUT the compiler won't know if it got the destroy the copied pointers pass in parameters or not if we dont specify a: lifetime
//  It can be ANY NAME, here I wrote "life" but it could be "chicken"
//  You need a single quote and the word, mostly look like generic with extra steps
//  Even the return signature need a SAME lifetime
fn gt<'life>(a: &'life f32, b: &'life f32) -> &'life f32 {
    if a > b {
        a   //  That way one of those two pointers won't be destroyed
    } else {
        b   //  and simply returned to the parent scope outside of the function
    }
}