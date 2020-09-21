fn main(){
    let elements = [5,15,20,25,30,35,40,45,50];

    let mut sum = 0;
    for i in elements.iter(){
        sum = sum + i;
    }
    println!("{}",sum);
}
