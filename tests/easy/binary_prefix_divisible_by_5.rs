// 1018. Binary Prefix Divisible By 5
// https://leetcode.com/problems/binary-prefix-divisible-by-5/

struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        println!("nums: {:?}", &nums);

        let mut output = vec![false; nums.len()];
        let mut x = 0;

        for i in 0..nums.len() {
            let n = nums[i];
            x = (x << 1 | n) % 5;
            output[i] = x == 0;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_prefix_divisible_by_5::Solution;

    #[test]
    fn test_prefixes_div_by5_1() {
        let nums = [0, 1, 1].to_vec();
        assert_eq!(vec![true, false, false], Solution::prefixes_div_by5(nums));
    }

    #[test]
    fn test_prefixes_div_by5_2() {
        let nums = [1, 1, 1].to_vec();
        assert_eq!(vec![false, false, false], Solution::prefixes_div_by5(nums));
    }

    #[test]
    fn test_prefixes_div_by5_3() {
        let nums = [
            1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0,
            0, 0, 1, 1, 0, 1, 0, 0, 0, 1,
        ]
        .to_vec();
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ],
            Solution::prefixes_div_by5(nums)
        );
    }

    #[test]
    fn test_prefixes_div_by5_4() {
        let nums = [
            1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1,
            1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0,
        ]
        .to_vec();
        assert_eq!(
            vec![
                false, false, true, false, false, false, false, false, false, false, true, true,
                true, true, true, true, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, false, true,
                false, false, true, false, false, true, true, true, true, true, true, true, false,
                false, true, false, false, false, false, true, true
            ],
            Solution::prefixes_div_by5(nums)
        );
    }
}
