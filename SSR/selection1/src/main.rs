use std::io;
fn main() {
    let stdin = io::stdin();
    let mut ageStr: String = String::new();
    println!(" How old are you? ");
    stdin.read_line(&mut ageStr);
    let age: i32 = ageStr.trim().parse().expect("an integer was expected");
    if age < 18 {
        println!(" No beer for you :(");
    } else {
        println!(" Beer for you :)")
    }
    
}
