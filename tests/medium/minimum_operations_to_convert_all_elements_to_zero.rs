// 3542. Minimum Operations to Convert All Elements to Zero
// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/

use std::collections::HashSet;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/editorial/
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut s: Vec<i32> = Vec::new();
        let mut res: i32 = 0;
        for a in nums {
            while s.last().map_or(false, |&x| x > a) {
                s.pop();
            }
            if a == 0 {
                continue;
            }
            if s.last().map_or(true, |&x| x < a) {
                res += 1;
                s.push(a);
            }
        }
        res
    }

    // Time Limit Exceeded
    pub fn _min_operations(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut op = 0;
        let mut values = nums
            .iter()
            .filter(|&n| *n > 0)
            .map(|n| *n)
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        values.sort_unstable();
        let mut nums = nums;

        for n in &values {
            let mut tmp = vec![];
            let mut change = 0;
            for s in &nums {
                if *s == *n {
                    change += 1;
                    if tmp.is_empty() || tmp[tmp.len() - 1] > 0 {
                        tmp.push(0);
                    }
                } else if *s == 0 && change > 0 {
                    change = 0;
                    op += 1;
                    tmp.push(0);
                } else {
                    tmp.push(*s);
                }
            }
            if change > 0 {
                op += 1;
            }
            nums = tmp;
        }

        op
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_convert_all_elements_to_zero::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = vec![0, 2];
        assert_eq!(1, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = vec![3, 1, 2, 1];
        assert_eq!(3, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = vec![1, 2, 1, 2, 1, 2];
        assert_eq!(4, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_4() {
        let nums = vec![3, 5, 2, 5, 2];
        assert_eq!(4, Solution::min_operations(nums));
    }
}
