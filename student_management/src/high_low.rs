pub fn high_low(grades: &Vec<i32>) -> [i32; 2] {
    let mut low_grade = 100;
    let mut high_grade = 0;
    for grade in grades.to_owned() {
        if grade < low_grade {
            low_grade = grade;
        }
        if grade > high_grade {
            high_grade = grade
        }
    }
    let high_low_grades: [i32; 2] = [low_grade, high_grade];
    return high_low_grades;
}
