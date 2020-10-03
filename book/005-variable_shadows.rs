fn main(){
    let a = 10;

    // Shadow of upper a
    let a = 20;
    println!("The value of a is: {}",a); // The value of a is: 20
}