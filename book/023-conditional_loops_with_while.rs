
// use std::io;
// fn main(){
//     println!("Enter City: ..");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Invalid input");

//     let clean_cities = ["Islamabad", "Pindi","Sawat","Kashmir","Quetta"];
// }

use std::io;
fn main(){
    println!("Enter any number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let mut int:i32 = input.trim().parse().unwrap();

    while int < 100 {
        int += 1;
        println!("{}",int);
    }
}