
// This is a simple Rust program that demonstrates variable immutability and mutability.
// It defines a constant and uses mutable and immutable variables.

fn main() {

    let first_name = "Alice";
    // Everything after the `let` keyword is immutable by default
    // first_name = "Bob"; // This line would cause a compile-time error
    let mut last_name = "Smith"; 
    println!("Hello, {} - {} !", first_name,last_name );
    last_name = "Plithara";
    println!("Hello, {} - {} !", first_name, last_name);
}



