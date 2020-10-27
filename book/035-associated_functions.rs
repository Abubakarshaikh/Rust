// Associated Functions: Without &self paramenter
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle{
    fn square(size: u32)-> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main(){

    let rect1 = Rectangle::square(3);
    println!("{:?}",rect1);
}
