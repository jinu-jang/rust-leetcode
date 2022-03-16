// https://leetcode.com/problems/two-sum/

pub trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

pub struct NSquared {}
impl Solution for NSquared {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx, i) in nums.iter().enumerate() {
            'inner: for (jdx, j) in nums.iter().enumerate() {
                if idx == jdx {
                    continue 'inner;
                }
                if i + j == target {
                    // return vec![idx as i32 , jdx as i32];
                    return vec![idx.try_into().unwrap() , jdx.try_into().unwrap()];
                }
            }
        }
        
        // Technically should never happen according to problem givens.
        unreachable!();
    }
}

use std::collections::HashMap;
pub struct O1{}
impl Solution for O1 {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::<i32, usize>::new();
        for (idx, cur_val) in nums.iter().enumerate() {
            match hashmap.get(&(target - cur_val)) {
                None => {
                    hashmap.insert(*cur_val, idx);
                }
                Some(matching_idx) => {
                    return vec![*matching_idx as i32, idx as i32];
                }
            }

        }
        
        // Technically should never happen according to problem givens.
        unreachable!();
    }
}

#[allow(unused_macros)]
macro_rules! twosum_tests {
    ($($name:ident: $type:ty,)*) => {
    $(
        mod $name {
            use super::*;

            #[test]
            fn case_1() {
                assert_eq!(<$type>::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
            }

            #[test]
            fn case_2() {
                assert_eq!(<$type>::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
            }

            #[test]
            fn case_3() {
                assert_eq!(<$type>::two_sum(vec![3, 3], 6), vec![0, 1]);
            }
        }
    )*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    twosum_tests! {
        n_squared: NSquared,
        o_1: O1,
    }
}