// 3318. Find X-Sum of All K-Long Subarrays I
// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/

use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        println!("nums: {:?}, k: {k}, x: {x}", &nums);

        let k = k as usize;
        let mut result = vec![];

        for i in 0..nums.len() {
            let end = i + k - 1;
            if end < nums.len() {
                result.push(Self::sum_x(&nums[i..=end], x));
            }
        }

        result
    }

    fn sum_x(slice: &[i32], x: i32) -> i32 {
        // println!("slice: {:?}", &slice);
        let mut counter = vec![0; 1 + *slice.iter().max().unwrap() as usize];
        for &n in slice {
            counter[n as usize] += 1;
        }
        let mut list = counter.iter().enumerate().collect::<Vec<_>>();
        list.sort_by(|(ai, an), (bi, bn)| match bn.cmp(an) {
            Ordering::Equal => bi.cmp(ai),
            _ => bn.cmp(an),
        });

        // println!("list: {:?}", &list);
        list.iter()
            .take(x as usize)
            .map(|(i, n)| *i as i32 * **n)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_x_sum_of_all_k_long_subarrays_i::Solution;

    #[test]
    fn test_find_x_sum_1() {
        let nums = [1, 1, 2, 2, 3, 4, 2, 3].to_vec();
        let k = 6;
        let x = 2;
        let result = [6, 10, 12].to_vec();
        assert_eq!(result, Solution::find_x_sum(nums, k, x));
    }

    #[test]
    fn test_find_x_sum_2() {
        let nums = [3, 8, 7, 8, 7, 5].to_vec();
        let k = 2;
        let x = 2;
        let result = [11, 15, 15, 15, 12].to_vec();
        assert_eq!(result, Solution::find_x_sum(nums, k, x));
    }
}
