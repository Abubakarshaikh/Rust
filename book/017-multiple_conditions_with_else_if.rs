use std::io;
fn main(){
    println!("Enter Any Vowel ..");
    let mut input_vowel = String::new();
    io::stdin().read_line(&mut input_vowel).expect("Invalid input");

    let vowel:String = input_vowel.trim().parse().unwrap();
    
    if vowel == "a"{
        println!("This is Vowel: {}",vowel);
    } else if vowel == "e"{
        println!("This is Vowel: {}",vowel);
    } else if vowel == "i"{
        println!("This is Vowel: {}",vowel);
    } else if vowel == "o"{
        println!("This is Vowel: {}",vowel);
    } else if vowel == "u"{
        println!("This is Vowel: {}",vowel);
    } else {
        println!("This is not Vowel ...");
    }
}