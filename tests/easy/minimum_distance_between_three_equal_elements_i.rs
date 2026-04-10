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
            .map(|values| {
                values
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        if i == values.len() - 1 {
                            *p - values[0]
                        } else {
                            values[i + 1] - *p
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut min: i32 = -1;
        for mut item in items {
            item.sort_unstable();
            min = if min == -1 {
                item.iter().take(3).sum::<usize>() as i32
            } else {
                min.min(item.iter().take(3).sum::<usize>() as i32)
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
}
