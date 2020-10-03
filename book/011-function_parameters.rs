use std::io;
fn main(){
    println!("Inter any Integers: ");
    let mut input_int = String::new();
    io::stdin().read_line(&mut input_int).expect("Invalid input");
    let integers:i32 = input_int.trim().parse().unwrap();

    println!("The factorial is : {}",factorial(integers));
}

fn factorial(n: i32) ->i32{

    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}