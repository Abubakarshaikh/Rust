use std::io;
fn main(){
    println!("Enter Email ..");
    let mut input_email = String::new();
    io::stdin().read_line(&mut input_email).expect("invalid input");
    let email:String = input_email.trim().parse().unwrap();

    println!("Enter Password ..");
    let mut input_pass = String::new();
    io::stdin().read_line(&mut input_pass).expect("invalid input");
    let pass:i32 = input_pass.trim().parse().unwrap();

    // if email == "abc@gmail.com" || pass == 123 {

    //     if email == "abc@gmail.com"{
    //         if pass == 123{
    //             println!("You are log in");
    //         } else {
    //             println!("Invalid password");
    //         }
    //     } else {
    //         println!("Invalid Email. ");
    //     }
    // } else {
    //     println!("Invalid email and Password");
    // }


}