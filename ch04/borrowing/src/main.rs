/* 
A **reference** is like a **pointer** in that it’s an address we can follow to access the data stored at that address;
 that data is owned by some other variable. 
 
 --> Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.
We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.


Note: 
   This is more or less what is happening in python, references for objects (especially for mutable objects)
   Python is pass-by-object-reference (or "pass-by-assignment").
   It behaves like pass-by-reference for mutable objects, and like pass-by-value for immutable objects,
   but technically, it's always passing references


--> Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
_




**Mutable references** have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
The benefit of having this restriction is that Rust can prevent **data races** at compile time.
A data race is similar to a race condition and happens when these three behaviors occur:

1) Two or more pointers access the same data at the same time.
2) At least one of the pointers is being used to write to the data.
3) There’s no mechanism being used to synchronize access to the data.
Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime;
Rust prevents this problem by refusing to compile code with data races!


Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed 
because no one who is just reading the data has the ability to affect anyone else’s reading of the data.



**Dangling References**
In languages with pointers, it’s easy to erroneously create a dangling pointer;
a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
In Rust, by contrast, the compiler guarantees that references will never be dangling references:
if you have a reference to some data, **the compiler will ensure that the data will not go out of scope before the reference to the data does.**


Let’s recap what we’ve discussed about references:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

*/

fn main(){
    println!("Reference and borrowing demo!");
    let greet = String::from("Hello from Rust Programming Language!");
    println!("{} with length, `#{}`", greet, greet_length(&greet));
    let mut x = String::from("foooo");
    let r1  = &mut x ;
    mutable_reference(r1); // r1 is no longer used after this
    // this is ok because r1 is not valid anymore
    let r2 = &mut x ;
    




}

fn greet_length(s : &String ) -> usize {
    s.len()
    } // Here, s goes out of scope. But because s does not have ownership of what
      // it refers to, the value is not dropped.


 // we can have only one mutable reference.     
fn mutable_reference( s : &mut String) {
    println!("If we use a mutable reference we can modify the underling string");
    println!("s entering the function is: `{}`", s);
    s.push_str(" bar bar");
    println!("ante now s is: `{}`", s);
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped, so its memory goes away.
  // Danger!