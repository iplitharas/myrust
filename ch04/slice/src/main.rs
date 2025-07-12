/*
Rust’s standard library includes a number of very useful data structures called collections.
Most other data types represent one specific value, but collections can contain multiple values.
Unlike the built-in array and tuple types, the data that these **collections point to is stored on the heap**,
which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

--> A vector allows you to store a variable number of values next to each other.
--> A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
--> A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

**Slices** let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.



Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call to clear uses the reference in word,
so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference 
in word from existing at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated 
an entire class of errors at compile time!


Summary
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
 
 The Rust language gives you control over your memory usage in the same way as other systems programming languages,
but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book.
Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
*/


fn main() {
    let greet = String::from("Hello, world!");
      let first = first_word(&greet);
      println!("First word is: `{}`", first);
    // if for some reasons we cleanup the string
    // also the slice will not be valid and 
    // we are going to have a compiler error
    // greet.clear();  
    let first_again = first_word_improved(&greet);
     println!("First word is: `{}`", first_again);
     let mut  numbers : [i32;6] = [0,1,2,3,4,5];
      mutable_slice(&mut numbers[..]);
      println!("numbers now are: {:?}", numbers);
}



// helper function to find the first word in a sentence
fn first_word(s : &String) ->  &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            println!("found first empty space at index: `{}`", i);
            return &s[0..i];
            }
        }
        &s[..]

}

// This is an improvement because we can pass &str (immutable slice) or an immutable reference of String
// The type of s here is &str: it’s a slice pointing to that specific point of the binary.
//  This is also why string literals are immutable; &str is an immutable reference.
fn first_word_improved(s : &str) ->  &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            println!("found first empty space at index: `{}`", i);
            return &s[0..i];
            }
        }
        &s[..]

}

fn mutable_slice(numbers: &mut [i32]){
    for (index, number) in numbers.iter_mut().enumerate(){
        println!("got number: {} and index: {}", number, index);
         *number = - *number;

    }
}