// is no good because of O(N^2) time complexity.
pub fn bubble_sort(unsortedGrades: Vec<i32>) -> Vec<i32> {
    let length: usize = unsortedGrades.len();
    let mut sortedArr: Vec<i32> = unsortedGrades;
    for i in 0..length - 1 {
        for j in 0..length - 1 {
            if sortedArr[j] > sortedArr[j + 1] {
                sortedArr.swap(j, j + 1);
            }
        }
    }
    return sortedArr;
}
