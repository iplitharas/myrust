/*

Rust has three kinds of loops: 
1)loop,
2) while, and
3) for.
Let’s try each one.

*/

fn main() {
    println!("Hello, world!");
    tenary_exp();
    for_loop_in_array_demo();
    for_loop_in_string();
    for_loop_with_range();
    loop_with_name();
    while_loop_demo()
}



fn tenary_exp(){

    let something = true  ;
    // is this tenary expression ?
    // No, this is not a ternary expression in Rust.
    // Rust does not have a ternary operator like some other languages (e.g., `condition ? true_value : false_value`).
    // Instead, Rust uses `if` expressions to
    // but it it's similar!
    let something_else = if something { "it's true"} else { "it's false"};
    println!("Condition is: `{}`",  something);
    println!("Someting else is: `{}`", something_else) 

}

fn for_loop_in_array_demo(){
    let numbers = [1,2,3,4,5]; 
    for number in numbers {
        println!("Number is: `{}`", number); 
    }

}


// range is similar with python starts from 1 until 5
fn for_loop_with_range(){
    for number in (1..5).rev() {
        println!("Number is: `{}`", number)

    }
}

fn loop_with_name(){
    let mut count = 0;  
    'counting_up_loop : loop {
        println!("Count is: {}", count);
        let  mut remaining = 10 ; 
        loop {    
            println!("remaining is: {}", remaining);   
            if remaining == 8 {
                break ;
            }
            if count == 3 {
                break 'counting_up_loop;
            }
            remaining -= 1 ; 
            count += 1 ;

       
    }

    }
}


fn while_loop_demo(){
    let numbers = [1,2,3,4,5];
    let mut counter = 0; 
    while true {
        if counter <= numbers.len() {
              println!("number is {}", numbers[counter] );
              counter +=1;

        } else {
            break 
        }
      
    }
}


fn for_loop_in_string(){
    let some_phrase = "To good to be true!";
    for char in some_phrase.chars() {
           println!("char is: `{}`", char); 

    }
}