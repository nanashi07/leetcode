// 3718. Smallest Missing Multiple of K
// https://leetcode.com/problems/smallest-missing-multiple-of-k/

struct Solution;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums
            .into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .into_iter()
            .filter(|&n| n >= k && n % k == 0)
            .collect::<Vec<_>>();
        nums.sort_unstable();

        for i in 0..nums.len() {
            let n = (i + 1) as i32 * k;
            if nums[i] != n {
                return n;
            }
        }
        (1 + nums.len()) as i32 * k
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_missing_multiple_of_k::Solution;

    #[test]
    fn test_missing_multiple_1() {
        let nums = [8, 2, 3, 4, 6].to_vec();
        let k = 2;
        assert_eq!(10, Solution::missing_multiple(nums, k));
    }

    #[test]
    fn test_missing_multiple_2() {
        let nums = [1, 4, 7, 10, 15].to_vec();
        let k = 5;
        assert_eq!(5, Solution::missing_multiple(nums, k));
    }

    #[test]
    fn test_missing_multiple_3() {
        let nums = [3, 29, 3, 51].to_vec();
        let k = 3;
        assert_eq!(6, Solution::missing_multiple(nums, k));
    }

    #[test]
    fn test_missing_multiple_4() {
        let nums = [
            83, 96, 34, 56, 48, 30, 7, 14, 77, 66, 66, 66, 21, 17, 38, 7, 9,
        ]
        .to_vec();
        let k = 7;
        assert_eq!(28, Solution::missing_multiple(nums, k));
    }
}
