// 2784. Check if Array is Good
// https://leetcode.com/problems/check-if-array-is-good/

struct Solution;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        print!("{:?}", &nums);
        let mut s = vec![0; nums.len() - 1];
        for n in &nums {
            let i = *n as usize - 1;
            if i >= s.len() {
                return false;
            }
            if s[i] == 0 {
                s[i] = *n;
            } else {
                if *n == s.len() as i32 && s[i] == *n {
                    s[i] = 2 * *n;
                } else {
                    return false;
                }
            }
        }
        s.binary_search(&0).is_err()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_array_is_good::Solution;

    #[test]
    fn test_is_good_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(false, Solution::is_good(nums));
    }

    #[test]
    fn test_is_good_2() {
        let nums = [1, 3, 3, 2].to_vec();
        assert_eq!(true, Solution::is_good(nums));
    }

    #[test]
    fn test_is_good_3() {
        let nums = [1, 1].to_vec();
        assert_eq!(true, Solution::is_good(nums));
    }

    #[test]
    fn test_is_good_4() {
        let nums = [3, 4, 4, 1, 2, 1].to_vec();
        assert_eq!(false, Solution::is_good(nums));
    }
}
