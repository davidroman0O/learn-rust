// We can import other libraries
// from crates.io, ex : https://crates.io/crates/rand
// You'll see Cargo.toml, copy/paste the content under your Cargo.toml under [dependencies] and run `cargo run`, you'll see it to download the lib
extern crate rand; // and then we can import it
                   // But it's not enough, we need to import one "a function or struct" or multiple parts "{}"
use rand::prelude::*; // * will load everything under "prelude" library

// Or import local modules
mod rogue;

// and create some with "mod"
mod great_module {
    fn greeting() {
        println!("Hello you");
    }
    pub fn public_greeting() {
        println!("Hello y'all");
    }
}

fn main() {
    //  By default, fn are private
    // great_module::
    //  You need to specify a "pub fn"
    great_module::public_greeting();

    // We added a folder under ./src/rogue/mod.rs
    rogue::attack();

    // let's use our imported lib with a function exposed
    // We use a generic signature, don't focus on it, we'll see it later
    println!("{}", random::<u32>())
}
