



fn main() {
    println!("Hello, world!");
    let user_1 = User{username : String::from("ioannispli"), email : String::from("johnplitharas@domain.com"), active : true };
    let user_1 = update_email(String::from("johnplitharas@other_domain.com"), user_1);
    println!("user email updated to: {}", user_1.email); // because String doesn't implement the copy, same as our struct we aren't able to use again the user_1 
    // because it changed ownership, moved to function and back 
    let red = Color(0,0,0);
    println!("red is: `{:?}`", red);
    let my_boolean = AlwaysEqual ; 

}

#[derive(Debug)]
struct Color(i32,i32,i32);


struct AlwaysEqual;

#[derive(Debug)]
struct User{
    username : String, 
    email : String, 
    active: bool


    
}


fn update_email(new_email : String, user : User ) -> User{ 
    User{email: new_email, ..user}
    
}