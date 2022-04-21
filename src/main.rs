use std::ptr::null;

mod egg;
mod merge;
mod search_matrix;
mod solution;

impl solution::Solution {
    pub fn is_palindrome(str: String) -> bool {
        if str.trim().is_empty() ||str.len() == 0 {
            return true;
        }
        let s = str.to_lowercase();

        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            while left < right && !s.chars().nth(left).unwrap().is_alphanumeric() {
                left += 1;
            }
            while left < right && !s.chars().nth(right).unwrap().is_alphanumeric() {
                right -= 1;
            }

            use std::cmp::Ordering;

            match s
                .chars()
                .nth(left)
                .unwrap()
                .to_lowercase()
                .cmp(s.chars().nth(right).unwrap().to_lowercase())
            {
                Ordering::Equal => {
                    left += 1;
                    right -= 1;
                },
                _default => {
                   return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    let result = solution::Solution::super_egg_drop(5, 100);
    println!("Hello, world! {}", result);

    print!(
        "is_palindrome {}",
        solution::Solution::is_palindrome("a.".to_string())
    );
}
