// Multiple impl Blocks
struct Rectangle {
    width:u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) ->u32{
        self.width * self.height
    }
}
impl Rectangle{
    fn can_hold(&self, other: Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle{
        width: 30,
        height: 40,
    };

    println!("{}",rect1.area());
    println!("{:?}",rect1.can_hold(rect2));
}
