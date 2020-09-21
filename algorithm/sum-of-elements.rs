// fn main(){
//     let elements = [5,15,20,25,30,35,40,45,50];

//     let mut sum = 0;
//     for i in elements.iter(){
//         sum = sum + i;
//     }
//     println!("{}",sum);
// }


fn main(){

    let arr = ["a","b","u","b","a","k","a","r"];

    let mut add = String::from("");
    for i in arr.iter(){

        add =  add + &i.to_string();
    }
    println!("{}",add);
}
