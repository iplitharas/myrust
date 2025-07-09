/*
Stack : 
    -> All data stored on the stack must have a known, **fixed size**.
    -> Data with an unknown size at compile time or a size that might change must be stored on the heap instead.


Heap: 
    The heap is less organized: when you put data on the heap, you request a certain amount of space.
    The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use,
    and returns a pointer, which is the address of that location.
    This process is called **allocating** on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).
    Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack,
    but when you want the actual data, you must follow the pointer.
    Think of being seated at a restaurant.
    When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there.
    If someone in your group comes late, they can ask where you’ve been seated to find you.


    
Outcome 1: 
Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data;
that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator 
must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.


Outcome 2:
Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

Outcome 3:
When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed 
onto the stack. When the function is over, those values get popped off the stack.


Conclusion:
Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out 
of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, 
but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.


Ownership Rules:

--> Each value in Rust has an owner.
--> There can only be one owner at a time.
--> When the owner goes out of scope, the value will be dropped.

*/

fn main() {
    println!("Onwnership in Rush!");
    variable_scope_demo();
    the_string_type_demo();
    move_demo_1();
    onwership_and_functions_1();
    onwership_and_functions_2();
}



// same as other proramming languages.
fn variable_scope_demo()
{                                                       // s is not valid in this line, is not declared yet.
    let s = "Ioannis";                    //  s is valid from this point forward
    println!("s is a string with value: {}", s);       // this scope is now over, and s is no longer valid

}

/* 
We are going to use a data type like string which is stored in heap
and see how ownership solves the problem of when it's time to cleanup it from the
heap.


String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. 
 One reason is that they’re **immutable**.
 
Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, 
Rust has a second string type, **String**. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 

*/
fn the_string_type_demo()
  {
    let mut s = String::from("Foo"); // s is valid from this point forward

    println!("string is: {}", s);
    s.push_str(" Bar");
    println!("string mutated and now is: {}", s)
} // this scope is now over, and s is no longer valid


/* 
Memory and Allocation
In the case of a **string literal**, we know the contents at compile time, so the text is hardcoded directly into the final executable.
This is why **string literals are fast and efficient**.
But these properties only come from the string literal’s immutability.
Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.



With the **String type**, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

The memory must be requested from the memory allocator at runtime.
We need a way of returning this memory to the allocator when we’re done with our String.
That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we
don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it,
just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, 
we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.


----> Rust
There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope.
When a variable goes out of scope, Rust calls a special function for us.
This function is called **drop**, and it’s where the author of String can put the code to return the memory.

--> Rust calls drop automatically at the closing curly bracket. 

*/




/* 
If you’ve heard the terms shallow copy and deep copy while working with other languages, 
the concept of copying the pointer, length, and
capacity without copying the data probably sounds like making a shallow copy. 
But because **Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move**
*/
fn move_demo_1(){
   let s1 = String::from("foo");
   let s2 = s1; // so what will happen here is that s1 will be moved to s2, meaning we do a shallow copy and invalid s1 
   println!("We aren't able to use s1 again, because onwership moved to s2");
   println!("But we are are able to use s2 normally, s2 is (the value from s1): `{}`", s2);



}



fn onwership_and_functions_1(){
    let s = String::from("foo string");
    println!("s has value: `{}`", s);
    take_onwership_from_something(s);

    // I cannot use s from this one because the onwersip gone, moved, move is shallow copy and first one invalid!
    let x = 10 ;
    take_copy(x);
    println!("I can use x again");
    let y = x + 10 ;
    println!("y is: `{}`", y);

}

fn take_onwership_from_something(s : String){
    println!("I toke the onwership and i will die because I don't return it and drop will be called!");
    println!("s has value: `{}`", s)

}


fn take_copy(s : i32) {
    println!("Everything that implement the copy or there are small enough with fixed length, const string,bool,int,tuple,array to be stored in the stack will not take onwership");
    println!("s in this case is a new copy in the stack, with value: `{}`", s);
}


fn onwership_and_functions_2(){
    let s1 = gives_ownership();
    println!("s took the onwership and has value: `{}`", s1);
    let s2 = String::from("hello"); 
    let s3 = take_and_give_back(s2);
    println!("I can use s3 because it's a move from s2: s3 is: `{}`", s3);
    // println!("Definetely I cannot use s1 again.. {}", s2 ); 
    // compiler gives all the info why we cannot, -> value borrowed here after move

} // s3 out of scope will be droped at this poing, s2 was moved (already dead), s1 goes out of scope and is dropped

fn gives_ownership() -> String{
      let s = String::from("bar string");
      s 

}

fn take_and_give_back(s: String) -> String{
    s
}

/* 
So with this apporach we have to always take ownership and returing back from functions
or 
Luckily for us, Rust has a feature for using a value without **transferring ownership**, called **references**.
*/