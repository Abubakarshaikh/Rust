// Structs let you create custom types that are meaningful for your domain
// By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
struct UniAddmisstionForm{
    name: String,
    lastname: String,
    age: i32,
}

fn main(){
    let uniform_01 = UniAddmisstionForm{
        name: String::from("Abubakar"),
        lastname: String::from("Shaikh"),
        age: 23,
    };

    let uniform_02 = UniAddmisstionForm{
        name: String::from("Umar farroque"),
        lastname: String::from("Shaikh"),
        age: 19,
    };

    let uniform_03 = UniAddmisstionForm{
        name: String::from("Umar farroque"),
        lastname: uniform_02.lastname,
        age: 19,
    };
}
