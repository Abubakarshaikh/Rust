
// use std::io;
// fn main(){
//     println!("Enter City: ..");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Invalid input");

//     let clean_cities = ["Islamabad", "Pindi","Sawat","Kashmir","Quetta"];
// }

// use std::io;
// fn main(){
//     println!("Enter any number");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Invalid input");
//     let mut int:i32 = input.trim().parse().unwrap();

//     while int < 100 {
//         int += 1;
//         println!("{}",int);
//     }
// }

// 1. while loop: 
// fn main(){
//     let mut i: i32 = 1; // Computer Memory save: 1 or 6

//     while i <= 5{ // code run
//         println!("i: {}",i); 
//         i += 1; 
//     } // return: 6

//     while i <= 3 {
//         println!("i: {}",i);
//         i += 1;
//     }
// }

// 2. nested while loop: 1
// fn main(){
    
//     let mut outer: i32 = 1;

//     while (outer <= 3){
//         println!("outer {}",outer);

//         let mut inner = 1;
//         while (inner <=5 ){
//             println!("inner: {}",inner);
//             inner += 1;
//         }
//         outer += 1;
//     }
// }

// 3. Nested while loop: 2
// fn main(){
//     let mut outer:i32 = 1;
//     let mut inner:i32 = 1; // when inner loop break, the value of inner is :6
//     while(outer <= 3){
//         println!("outer {}",outer);

//         while(inner <= 5){
//             println!("inner: {}",inner);
//             inner += 1;
//         }// inner while
//         outer += 1;
//     } //outer while
// }

// 4. Nested while loop: 3
fn main(){
    let mut outer:i32 = 1;
    let mut inner:i32 = 1; // 6
    let mut deep:i32 = 1; //: 4

    while (outer <= 3){
        println!("outer: {}",outer);

        while(inner <= 5){
            println!("inner: {}",inner);

            while(deep <= 3){
                println!("deep: {}",deep);
                deep += 1;
            } // deep
            inner += 1;
        } //inner
        outer += 1;
    } // outer
}
