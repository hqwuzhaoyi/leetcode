
use std::cmp::Ordering;
use crate::solution;

impl solution::Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut temp = Vec::new();
        temp.resize(m as usize + n as usize, 0);
        let mut index = 0;
        let mut i = 0 as usize;
        let mut j = 0 as usize;

        while i < m.try_into().unwrap() && j < n.try_into().unwrap() {
            match nums1[i].cmp(&nums2[j]) {
                Ordering::Equal | Ordering::Less => {
                    temp[index] = nums1[i];
                    index += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    temp[index] = nums2[j];
                    index += 1;
                    j += 1;
                }
            }
        }
        while i < m.try_into().unwrap() {
            temp[index] = nums1[i];
            index += 1;
            i += 1;
        }
        while j < n.try_into().unwrap() {
            temp[index] = nums2[j];
            index += 1;
            j += 1;
        }
        for (i, el) in temp.iter().enumerate() {
            nums1[i] = *el;
        }
    }
}
