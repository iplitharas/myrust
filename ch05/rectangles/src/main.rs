/*

Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!,
 which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).

**Method Syntax**
Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, 
and they contain some code that’s run when the method is called from somewhere else.
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapter 6 and Chapter 18, respectively),
and their **first parameter is always self**, which represents the instance of the struct the method is being called on.


**Defining Methods**

 The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
 
-> Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.
-> Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance, just as we did in rectangle: &Rectangle.
-> Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. 
If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. 

-> Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self 
into something else and you want to prevent the caller from using the original instance after the transformation.


Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else.
Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.
**Getters** are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API.


**Associated Functions** 
All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. 
We’ve already used one function like this: the String::from function that’s defined on the String type.

Associated functions that aren’t methods are often used for constructors **that will return a new instance of the struct**.
These are often called new, but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that 
would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice

**Multiple impl Blocks**
Each struct is allowed to have multiple impl blocks. 

**Summary**
Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected 
to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods
are a kind of associated function that let you specify the behavior that instances of your structs have.

But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.

*/

#[derive(Debug)]
struct Rectangle{
    width : u32, 
    height : u32
}

fn main() {
    let r1 = Rectangle{width: 30, height : 30};
    let area = r1.area(); // We pass a reference to r1 so we can borrow it without transferring ownership.
                                // This allows us to use r1 again after calling area().

    println!("The area of rectange: {r1:#?} is: {area}");
    println!("The width is: {} and is valid: {}", r1.width, r1.width());
    let r2 = Rectangle{width: 25, height: 25};
    println!("Can r1 {:?} hold r2: {:?} ? {}", r1,r2, r1.can_hold(&r2)); 
    println!("Can r2 {:?} hold r1: {:?} ? {}", r2, r1, r2.can_hold(&r1));

    // Create a new sqaure from Rectangle (no inheritance needed)
    let s1 = Rectangle::square(50);
    println!("Sqaure is: {:?}", s1);
}


impl Rectangle {
    fn area(&self) -> u32 {
    self.width * self.height
}
   fn width(&self) -> bool {
      return self.width > 0 

   }

   fn can_hold(&self, other : &Rectangle) -> bool {
      self.width > other.width && self.height > other.height

   }
   fn square(size : u32) -> Self {
    Self{width: size, height: size}
   }
    
}
