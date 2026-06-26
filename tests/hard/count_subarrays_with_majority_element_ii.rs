// 3739. Count Subarrays With Majority Element II
// https://leetcode.com/problems/count-subarrays-with-majority-element-ii/

struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        let offset = n as i32 + 1;
        let size = 2 * n + 3;
        let mut bit = vec![0i64; size + 1];

        let update = |bit: &mut Vec<i64>, mut i: usize, v: i64| {
            while i <= size {
                bit[i] += v;
                i += i & i.wrapping_neg();
            }
        };

        let query = |bit: &Vec<i64>, mut i: usize| -> i64 {
            let mut s = 0i64;
            while i > 0 {
                s += bit[i];
                i -= i & i.wrapping_neg();
            }
            s
        };

        let mut result = 0i64;
        let mut prefix = 0i32;

        update(&mut bit, (prefix + offset) as usize, 1);

        for &x in &nums {
            prefix += if x == target { 1 } else { -1 };
            let idx = (prefix + offset) as usize;
            result += query(&bit, idx - 1);
            update(&mut bit, idx, 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_subarrays_with_majority_element_ii::Solution;

    #[test]
    fn test_count_majority_subarrays_1() {
        let nums = [1, 2, 2, 3].to_vec();
        let target = 2;
        assert_eq!(5, Solution::count_majority_subarrays(nums, target));
    }

    #[test]
    fn test_count_majority_subarrays_2() {
        let nums = [1, 1, 1, 1].to_vec();
        let target = 1;
        assert_eq!(10, Solution::count_majority_subarrays(nums, target));
    }

    #[test]
    fn test_count_majority_subarrays_3() {
        let nums = [1, 2, 3].to_vec();
        let target = 4;
        assert_eq!(0, Solution::count_majority_subarrays(nums, target));
    }
}
