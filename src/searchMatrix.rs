use std::cmp::Ordering;
use crate::solution;
impl solution::Solution {
    pub fn searchMatrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut x, mut y) = (0, matrix[0].len() - 1);
        while x < matrix.len() && y < matrix[0].len() {
            match matrix[x][y].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => y -= 1,
                Ordering::Less => x += 1,
            }
        }
        false
    }
}
