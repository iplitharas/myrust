use std::io;
fn main() {
    arrays_demo()
   
}


fn arrays_demo(){
    let mut numbers : [i32; 5 ] = [1,2,3,4,5];
    // print array 
    println!("The numbers are: {:#?}", numbers);
    let first_number = numbers[0];
    println!("The first number is: {}", first_number);
    numbers[0] = 10; 
    println!("The numbers after change: {:#?}", numbers);
    println!("The length of the array is: {}", numbers.len());
    
    let mut index = String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line!");

    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = numbers[index];

    println!("The value of the element at index {index} is: {element}");

}

