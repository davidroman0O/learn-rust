

fn main() {
    
    //  You're already familiar with owenship
    //  Go back to the related section if needed
    //  Let's see one more case

    let base = String::from("I'm just a variable");
    {
        //  Instead of just passing the variable... that aka a pointer (&str)
        //  we clone the pointer, so is the variable
        let gimme = base.clone();
        //  That way, we can manupulate the copy of "base"
        println!("{}", gimme);
    }
    // And "base" is usage outside of the scope since it was only copied
    println!("{}", base);
    //  Remember that is you only passe the variable
    //  let gimme = base;  like this
    //  the pointer will be pass to "gimme", so destroy at the end of the bloc
    //  so inusable on println!("{}", base); because the pointer doesn't exists anymore

    //  That's an important concept, go back to "ownership" section
    //  come back here, try to play with it to understand it if needed

    //  But we can't do the same kind of thing by "borrowing" the pointer
    let reference = String::from("Use Rust, you will");
    {
        //  That mean we have a reference of "renference"
        let useit = &reference; // aka smart pointer
        let thereisanother = &reference;
        println!("{}", useit);
        println!("{}", thereisanother);
        //  "useit" will be destroyed but the main "reference" variable won't
    }
    println!("{}", reference);
    //  The pointer in Rust are "smart pointers" managed  by the compiler

    //  AND you can have a mutable variable
    let mut mutateme = String::from("notice me senpai");
    {
        let immutable_reference = &mutateme;
        
        //  You can create multiple imutable pointer reference BEFORE &mut
        let i_am_useless = &mutateme;
        
        //  Change have this line of code since this variable receive a immutable pointer
        // immutable_reference.push_str(", notice me!");
        
        //  If you add &mut you'll have a mutable pointer
        //  It's like copying the pointer of "mutateme" into "mutate_reference"
        //  This is this the borowing mecanism
        let mutate_reference = &mut mutateme;
        
        //  You can't create it because it as been borrowed as mutable right before
        //  let i_will_fail = &mutateme;

        //  And this will works
        mutate_reference.push_str(", notice me!");
        println!("{}", mutate_reference);
    }
    println!("{}", mutateme);



}
