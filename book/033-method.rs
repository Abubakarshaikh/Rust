// Methods let you specify the behavior that instances of your structs have
struct Rectangle{
    width: i32,
    height: i32,
}

impl Rectangle{
    fn area(&self) ->i32{
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 29,
        height: 49,
    };

    println!("The area of rectangle is {} sqaure in pixel",rect1.area());
}
