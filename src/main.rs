mod solution;
mod egg;
mod merge;
mod searchMatrix;

fn main() {
    let result = solution::Solution::super_egg_drop(5, 100);
    println!("Hello, world! {}", result);
}
