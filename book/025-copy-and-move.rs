// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.// 
// All the floating point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

// copy

fn main(){
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);

}

// move
fn main(){
    let x = String::from("Hello, world");
    let y = x; // x is move
    // x is not valid 
    
    println!("x = {}, y = {}", x, y);
}
