fn main() {
    // ----- Ownership Rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // ==========================================================

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Move (not shallow copy)

    println!("{}, world!", s1);

    // ==========================================================

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); This doesn't work because the ownership was moved.

    let x = 5;
    makes_copy(x);
    println!("{}", x); // This works because simple variables like integer have copy trait.

    // ==========================================================

    let s1 = gives_onwership(); // This returns String with the ownership.
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    // ==========================================================

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing
    println!("The length of '{}' is {}.", s1, len);

    // ========================================================== 

    // When you want to modify without taking the ownership
    let mut s1 = String::from("hello");
    change(&mut s1);

    // Mutable Reference has a restriction which is you can only have one mutable referece
    // to a particular piece of data in a particular scope
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r2 = &mut s; <- error
    // Rust can prevent "data races" at compile time.
    // Data race occurs if we have two pointers pointing to the same piece of data and
    // one of those pointers is used to write to the data and there's no mechanism to synchronize
    // data access between those pointers.
    // In that situation, you could imagine that one pointer will try to read the data
    // in the middle of the other pointer modifying the data and in that case we'll get corrupt data back.

    // let r3 = &mut s; <- error
    // You can't have a mutable reference if an immutable reference already exists.

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // This works because r1 and r2 dropped on the above line.
    println!("{}", r3);

    // ==========================================================
    
    // Dangling reference
    // let reference_to_nothing = dangle();

    // ==========================================================

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_onwership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { <- This occurs an error because "s" will drop after this function and the value will be invalid.
//     let s = String::from("hello");

//     &s
// }