/*
============================================================================
               The match Control Flow Construct
============================================================================
Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
Patterns can be made up of literal values, variable names, wildcards, and many other things;
The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Syntax:

match (AnyType)

Next are the match arms.
An arm has two parts: a pattern and some code.
 
Each arm is separated from the next with a comma.

When the match expression executes, it compares the resultant value against the pattern of each arm, in order.
If a pattern matches the value, the code associated with that pattern is executed.
If that pattern doesn’t match the value, execution continues to the next arm,
much as in a coin-sorting machine. We can have as many arms as we need

============================================================================
            Patterns That Bind to Values
============================================================================
Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. 
This is how we can extract values out of enum variants. 

============================================================================
        Matching with Option<T>
============================================================================
Does Some(5) match Some(i)? It does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. 
The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.

=============================================================================
       Matches Are Exhaustive
=============================================================================
This don't match the None so it will not compile..
```
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

```
===============================================================================
        Catch-All Patterns and the _ Placeholder
===============================================================================
Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value.
This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

Let’s change the rules of the game: now, if you roll anything other than a 3 or a 7, you must roll again.
We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other:

If we want to use the variable we can use `other`

```
match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

```

*/

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
   println!("A quarter of coin is: #`{}` cents", value_in_cents(Coin::Quarter(UsState::Alabama)));
   println!("A dime of coin is: #`{}` cents", value_in_cents(Coin::Dime));
   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);
   default_match(5);
 


}

fn value_in_cents(coin : Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }

}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None, 
        Some(i) => Some(i+1 )
    }
}


fn default_match(dice : u8) {
    match dice {
        3 => println!("You won with a 3"),
        4 => println!("You won with a 4 "),
        _ => println!("You are out of game..")
    }
}