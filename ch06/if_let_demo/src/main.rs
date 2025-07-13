/*
===============================================================
        Concise Control Flow with if let and let else
===============================================================
The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.


==================== let .. else ==============================
 Rust has let...else. The let...else syntax takes a pattern on the left side and an expression on the right, very similar to if let, 
 but it does not have an if branch, only an else branch. If the pattern matches, it will bind the value from the pattern in the outer scope. 
 If the pattern does not match, the program will flow into the else arm, which must return from the function.
==============================================================
*/

enum Coin{
    Quarter(UsState),
    Peni,
}
 #[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn main() {
    // instead of doing this we can
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    };

    if let Some(max) = config_max {
        println!("The maximum is configured to be: `{}`", max);
    }
    let name: Option<String> = None ;
    if let Some(name) = name {
        println!("Name is: {}", name);
    } else {
        println!("Name is None: {:?}", name);
    }
    // let .. else
    greet_someone(None);
    greet_someone(Some(String::from("Ioannis")));
  

}

// use let..else
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// checking first with if..let
fn describe_state_quarter_odd(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}




fn greet_someone(name: Option<String>) {
    let Some(name) = name else {
        println!("No name provided.");
        return;
    };

    println!("Hello, {}!", name);
}


