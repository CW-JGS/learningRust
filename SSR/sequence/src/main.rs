use std::io::{self};
fn main() {
    let mut firstName = String::new();
    let mut surname: String = String::new();
    let mut stdIDStr: String = String::new();
    let mut yearOfBirthStr: String = String::new();
    let mut currentYearStr: String = String::new();
    let mut stdin = io::stdin();
    println!(" enter your first name : ");
    stdin.read_line(&mut firstName);
    println!(" enter your surname : ");
    stdin.read_line(&mut surname);
    println!(" enter student id : ");
    stdin.read_line(&mut stdIDStr);
    let studentID: i32 = stdIDStr.trim().parse().expect("input was not an integer");
    println!(" enter the year you were born : ");
    stdin.read_line(&mut yearOfBirthStr);
    let yearOfBirth: i32 = yearOfBirthStr
        .trim()
        .parse()
        .expect("Value was not an integer");
    println!(" enter the current year");
    stdin.read_line(&mut currentYearStr);
    let currentYear: i32 = currentYearStr
        .trim()
        .parse()
        .expect("value is not an integer");
    println!("{} {}", firstName, surname);
    println!("{}", (currentYear - yearOfBirth));
}
