use std::io;
fn main(){
    println!("Enter any number 1 to 10");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let input_int:i32 = input.trim().parse().unwrap();

    let mut a = 1;
    let mut b = 1;
    let mut res = 0;

    println!("");
    for i in 1..(input_int + 1){
        res = b + a; //first: 2, second: 3, third: 5, fourth: 8, fifth: 13,
        b = a; //first: 1, second: 2, third: 3, fourth: 5, fifth: 8,
        a = res; //first: 2, second: 3, third: 5, fourth: 8 fifth: 13,
        println!("{}",a);
    }
}