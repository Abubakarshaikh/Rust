// if we try to modify something we’re borrowing: 
// cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn main(){
//     let s1 = String::from("Hello");
//     change(&s1);
// }

// fn change(some_string: &String){
//     some_string.push_str(", world");
// }




fn main(){
    let mut  s1 = String::from("Hello");
    change(&mut s1);
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

// multiple mutable refference
// 1. cannot borrow `s1` as mutable more than once at a time
// 2. cannot borrow `s1` as mutable because it is also borrowed as immutable

// 1. 
// fn main(){

//     let mut s1 = String::from("Hello");
//     // let s2 = &s1; // Immutbale form: cannot modify
//     let s2 = &mut s1; // mutbale form: can be modify
//     let s3 = &mut s1; // cannot borrow `s1` as mutable more than once at a time


//     s2.push_str(", world");

//     println!("{}",s2);
// }

// solution
fn main(){

    let mut s1 = String::from("Hello");
    { // s2 is not valid
    let s2 = &mut s1; // mutbale form: can be modify
    s2.push_str(", world"); // s2 is valid
    println!("{}",s2); // s2 is valid
    } // s2 is no longer valid

    let s3 = &mut s1;
    s3.push_str(", world pakistan");
    println!("{}",s3);
}

// 2. 
// fn main(){
//     let mut s1 = String::from("Hello");
//     let s2 = &s1;
//     let s3 = &mut s1;

//     println!("{}  {}",s2,s3);
// }

// solution
// fn main(){
//     let mut s1 = String::from("Hello");

//     // s2 is not valid
//     { // s2 is not valid
//     let s2 = &s1; // s2 is valid
//     println!("{} ",s2); // s2 is valid
//     } // scop end s2 is not vaid
//     let s3 = &mut s1;

//     println!("{} ",s3);
// }
