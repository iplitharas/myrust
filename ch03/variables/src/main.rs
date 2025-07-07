/*
Everything in Rust is immutable by default.
We can use the mut keyword to make a variable mutable.
We can also shadow a variable by using the let keyword again.
Shadowing allows us to change the value of a variable without making it mutable.

for example:
```
let spaces = "   ";
let spaces = spaces.len();
```
The first spaces variable is a string type and the second spaces variable is a number type.
 Shadowing thus spares us from having to come up with different names,
  such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.

*/
const DAY_COUNTDOWN_SEC: u32 = 24 * 60 * 60 ;


fn main() {
    println!("Variables demo 1");
    variables_mutate();
    let x = "[Global scope] I have the value of ten";
    shadowing();
}


// I cant mutate it's value 
// I cannot mutate it's type
fn variables_mutate(){
    let mut x = 5.0 ;
    println!("x is: `{}`", x );
    x = 6.3;
    println!("x is: `{}`", x );
    println!("Total remaining seconds are: `{}`s",DAY_COUNTDOWN_SEC )
}

/*
Shadowing is different from marking a variable as mut because 
we’ll get a compile-time error if we accidentally try to reassign
to this variable without using the let keyword. 
By using let,  we can perform a few transformations on a value but have the variable
be immutable after those transformations have been completed.
*/

fn shadowing(){
    println!("Shadowing demo");
    let x = 1 ;
    let x = x + 5 ;
     println!("x is: `{}`", x );
     {
        let x = x * 2 ;
        println!("shadowing in inner scope");
        println!("x now  is: `{}`", x );
     }
     println!("And x is: `{}`", x );


    

}