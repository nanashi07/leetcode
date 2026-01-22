// 3507. Minimum Pair Removal to Sort Array I
// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        println!("nums: {nums:?}");

        let mut count = 0;
        let mut nums = nums;

        while !Self::is_sorted(&nums) {
            let sums = &nums
                .iter()
                .take(nums.len() - 1)
                .enumerate()
                .map(|(i, n)| *n + nums[i + 1])
                .collect::<Vec<_>>();
            let min = sums.iter().min().unwrap();
            for i in 0..sums.len() {
                if *min == sums[i] {
                    let c = nums.clone();
                    nums.remove(i + 1);
                    nums[i] = *min;
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    fn is_sorted(nums: &Vec<i32>) -> bool {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        sorted.eq(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_pair_removal_to_sort_array_i::Solution;

    #[test]
    fn test_minimum_pair_removal_1() {
        let nums = [5, 2, 3, 1].to_vec();
        assert_eq!(2, Solution::minimum_pair_removal(nums));
    }

    #[test]
    fn test_minimum_pair_removal_2() {
        let nums = [1, 2, 2].to_vec();
        assert_eq!(0, Solution::minimum_pair_removal(nums));
    }

    #[test]
    fn test_minimum_pair_removal_3() {
        let nums = [-2, 1, 2, -1, -1, -2, -2, -1, -1, 1, 1].to_vec();
        assert_eq!(10, Solution::minimum_pair_removal(nums));
    }
}
