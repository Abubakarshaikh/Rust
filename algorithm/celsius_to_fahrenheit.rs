use std::io;
fn main(){
    println!("Celsius to Fahrenheit OR Fahrenheit to Celsius?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    let input:i32 = input.trim().parse().unwrap();

    let cel_to_fehr = input/5 * 9 + 32;
    let fehr_to_cel = 32 - input * 5 / 9;
    println!("Fahrenheit: {}",cel_to_fehr);
    println!("Celsius: {}",fehr_to_cel);
}
