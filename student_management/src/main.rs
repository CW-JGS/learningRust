use std::io;

use high_low::high_low;
use student::Student;
mod average;
mod high_low;
mod sort;
mod student;
fn main() {
    let stdin = io::stdin();
    let mut io_string: String = String::new();
    let mut students: Vec<student::Student> = Vec::new();
    let mut students_to_enter: i32;
    stdin.read_line(&mut io_string);
    students_to_enter = io_string
        .trim()
        .parse::<i32>()
        .expect("expected an integer");
    for i in 0..students_to_enter {
        let mut full_name = String::new();
        let mut grades: Vec<i32> = Vec::new();
        let mut num_grades: i32;
        stdin.read_line(&mut full_name);
        stdin.read_line(&mut io_string);
        num_grades = io_string.trim().parse().expect("expected an integer");
        for j in 0..num_grades {
            stdin.read_line(&mut io_string);
            grades.push(io_string.trim().parse().expect("expected an integer"));
        }
        let mut sorted: Vec<i32> = sort::bubble_sort(grades);
        let mut average = average::average(&sorted);
        let mut high_low: [i32; 2] = high_low(&sorted);
        students.push(Student {
            name: full_name,
            grades: sorted,
            average: average,
            high_low: high_low,
        });
    }
    for std in students {
        println!("{} {:?} {}", std.name, std.grades, std.average)
    }
}
