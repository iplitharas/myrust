/*
==================================================================================================================================================
** Enums and Pattern Matching** 
* Enums allow you to define a type by enumerating its possible variants.
 * Next, we’ll explore a particularly **useful enum, called Option**, which expresses that a value can be either something or nothing.

 `enums`` give you a way of saying a value is **one of a possible set of values**.


--> rather than an enum inside a struct, we can put data directly into each enum variant. 
The name of each enum variant that we define also becomes a function that constructs an instance of the enum.
That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
We automatically get this constructor function defined as a result of defining the enum.


--> you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.


==================================================================================================================================================

** The Option Enum and Its Advantages Over Null Values **

Programming language design is often thought of in terms of which features you include, but the features you exclude are important too.
-> Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there.
-> In languages with null, variables can always be in one of two states: null or not-null.

We don't need to define it. we just use Some
enum Option<T> {
    None,
    Some(T),
}

Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding 
Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.


When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value.
Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

This doen't compile
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;

131 |     let sum = x + y;
    |                 ^ no implementation for `i8 + Option<i8>`

In other words, you have to **convert an Option<T> to a T** before you can perform T operations with it. 
Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

==================================================================================================================================================
*/

 #[derive(Debug)]
enum IpAddrKind{
    V4, 
    V6,
}



 #[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32,y :i32}, // Like a named-field struct
    Write(String),
    ChangeColor(i32, i32,i32),
}

/*
The Enum Message is equivelant with:

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct



But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with 
the Message enum defined in Listing 6-2, which is a single type.
*/


impl  Message {
   fn call(&self){
      println!("Enum method called with self is `{:?}`", self);
   }    
}

 #[derive(Debug)]
enum IPAddrKindVersion2{
    V4(String), // this is a function call 
    V6(String), // same for this function call 
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Ip address of version4 is: `{:?}`", four);
    println!("Ip address of version6 is: `{:?}`", six);
    route_based_on_ip_version(four);

    let home = IPAddrKindVersion2::V4(String::from("127.0.0.1"));
    let loopback = IPAddrKindVersion2::V6(String::from("::1"));
    println!("Ip address of home is `{:?}`", home);
    println!("Ip address of loopback is `{:?}`", loopback);

    let message_move = Message::Move { x: 5, y: 5 };
    println!("Message move is: `{:?}`", message_move);
    message_move.call();
    let message_write = Message::Write(String::from("Hell from enum"));
    message_write.call();
    let message_quit = Message::Quit;
    message_quit.call();
    let message_color = Message::ChangeColor(123, 456, 789);
    message_color.call();
    let new_move = Message::Move { x: -10, y: 30 };
    new_move.call();

    let some_string = Some(String::from("Optional string"));
    println!("Some string is: {:?}", some_string);

    let some_missing_string : Option<String> = None;
    println!("Some missing string: {:?}", some_missing_string);
    let x: i8 = 5;
    let y: Option<i8> = None;
    match y {
        Some(value) => {
            let sum = x + value;
            println!("Sum is: {}", sum);
        }
        None => {
            println!("y is None");
        }
    }
    
     
    

     



       

}


fn route_based_on_ip_version(ip_version : IpAddrKind) {
    match ip_version {
        IpAddrKind::V4 => {println!("IP version 4 used")},
        IpAddrKind::V6 => {
            println!("IP version 6 used")
        }

    }
}