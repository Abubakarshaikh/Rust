//With ownership , Without refference
// fn main(){
    // let s1 = String::from("Hello, world"); // s1 valid
    // let len = calculate_length(s1); // s1 give ownership to calculate_length
                                    // s1 no longer valid

    // println!("{}",s1); // error because s1 is not valid
// }

// fn calculate_length(s: String) ->usize{
//     s.len()
// }


//with reffernce , without give ownership
fn main(){
    let s1 = String::from("Hello, world"); // s1 is valid
    let len = calculate_length(&s1); // s1 is valid , give reffence to calculate_length
                                    // s1 is valid
    println!("The length of '{}' is {}",s1,len);

}

fn calculate_length(s: &String) ->usize{
    s.len()
}
