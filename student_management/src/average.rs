pub fn average(grades: &Vec<i32>) -> f32 {
    let len = &grades.len();
    let mut bucket: i32 = 0;
    for grade in grades {
        bucket += grade;
    }
    return bucket as f32
        / len
            .to_string()
            .trim()
            .parse::<f32>()
            .expect("an error has occured");
}
