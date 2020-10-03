fn main(){
    let x = 5; // x, scop start
    println!("The value of x is {}",x); //end
    x = 6;

    //cannot assign twice to immutable variable `x`
    println!("The value of x is {}",x);
}