fn main(){


    let data1 = "Hello, world"; // Stack:copy
    let copyy = data1;
    // println!("data1 is :{} and copyy is {}",data1,copyy);
   
    let data2 = String::from("Hello, world"); // Heap:move
    let movee = data2;
    println!("data2 is :{} and copyy is {}",data2,movee);

}
