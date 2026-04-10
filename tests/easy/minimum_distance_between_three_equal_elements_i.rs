// 3740. Minimum Distance Between Three Equal Elements I
// https://leetcode.com/problems/minimum-distance-between-three-equal-elements-i/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            map.entry(*n).and_modify(|c| c.push(i)).or_insert(vec![i]);
        }
        let items = map
            .values()
            .filter(|values| values.len() >= 3)
            .collect::<Vec<_>>();

        let mut min: i32 = -1;
        for item in items {
            for w in item.windows(3) {
                let dist = 2 * (w[2] - w[0]) as i32;
                if min == -1 || dist < min {
                    min = dist;
                }
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_distance_between_three_equal_elements_i::Solution;

    #[test]
    fn test_minimum_distance_1() {
        let nums = [1, 2, 1, 1, 3].to_vec();
        assert_eq!(6, Solution::minimum_distance(nums));
    }

    #[test]
    fn test_minimum_distance_2() {
        let nums = [1, 1, 2, 3, 2, 1, 2].to_vec();
        assert_eq!(8, Solution::minimum_distance(nums));
    }

    #[test]
    fn test_minimum_distance_3() {
        let nums = [1].to_vec();
        assert_eq!(-1, Solution::minimum_distance(nums));
    }

    #[test]
    fn test_minimum_distance_4() {
        let nums = [1, 1, 1, 1].to_vec();
        assert_eq!(4, Solution::minimum_distance(nums));
    }
}
