// # 3201. Find the Maximum Length of Valid Subsequence I
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        println!("num: {:?}", &nums);

        let mut all_even_max = 0;
        let mut all_odds_max = 0;
        let mut even_odds_max = 1;

        for i in 0..nums.len() {
            let even = nums[i] % 2 == 0;

            if even {
                all_even_max += 1;
                if i > 0 && nums[i - 1] % 2 == 1 {
                    even_odds_max += 1;
                }
            } else {
                all_odds_max += 1;
                if i > 0 && nums[i - 1] % 2 == 0 {
                    even_odds_max += 1;
                }
            }
        }

        *[all_even_max, all_odds_max, even_odds_max]
            .iter()
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_maximum_length_of_valid_subsequence_i::Solution;

    #[test]
    fn test_maximum_length_1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(4, Solution::maximum_length(nums));
    }

    #[test]
    fn test_maximum_length_2() {
        let nums = [1, 2, 1, 1, 2, 1, 2].to_vec();
        assert_eq!(6, Solution::maximum_length(nums));
    }

    #[test]
    fn test_maximum_length_3() {
        let nums = [1, 3].to_vec();
        assert_eq!(2, Solution::maximum_length(nums));
    }
}
