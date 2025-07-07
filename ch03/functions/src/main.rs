/* 

**Statements**  are instructions that perform some action and do not return a value.
**Expressions** evaluate to a resultant value. 

Expressions do not include ending semicolons. 
If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
Keep this in mind as you explore function return values and expressions next.


Return statements:
in Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
You can return early from a function by using the return keyword and specifying a value,
but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

*/


fn main() {
    println!("Chapter 04 - Functions in Rust!");
    expr_demo();
    let numbers = create_five_numbers();
    println!("Numbers are: {:?}", numbers);
    let (first,second) = create_a_tuple();
    println!("After tuple un-packing first is: `{}` and second is: `{}`", first,second);
    println!("Created borrowed string, read only: {}", create_a_borrowed_string());
    println!("Created owned string, {}", create_a_owned_string());
}



fn expr_demo(){
   let mut x = {
    let y: i32 = 1;
    y +15   
   };
   // x -> 16    
   println!("x has value: {}", x);
  // x -> 46   
   x = x + 30;
println!("x has value: {}", x);



}


fn create_five_numbers() -> [i32; 5 ]{
    println!("Creating an array and returing it.");
    [3; 5]
}

fn create_a_tuple() -> (char,char) {
    ('a', 'b')
}


// borrowed string
fn create_a_borrowed_string() -> &'static str {
    "Ioannis"
}
fn create_a_owned_string() -> String {
    String::from("Ioannis")
}