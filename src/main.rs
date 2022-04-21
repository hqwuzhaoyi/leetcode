use std::cmp;
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn drop_egg(k: i32, t: i32, dp: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(floor) = dp.get(&(k, t)) {
            return *floor;
        }
        if k == 1 || t == 1 {
            return t;
        }
        let floor = Self::drop_egg(k, t - 1, dp) + Self::drop_egg(k - 1, t - 1, dp) + 1;
        dp.insert((k, t), floor);
        floor
    }

    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        if k == 1 {
            return n;
        }
        let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
        let (mut left, mut right) = (1, n + 1);

        let t = loop {
            let mid = left + (right as f32 - left as f32).log2() as i32;
            let floor = Self::drop_egg(k, mid, &mut dp);
            if floor > n {
                right = mid;
            } else if floor < n {
                left = mid + 1;
            } else {
                break mid;
            }
            if left == right {
                break left;
            }
        };

        t
    }
}

fn main() {
    let result = Solution::super_egg_drop(5, 100);
    println!("Hello, world! {}", result);
}
