// 66. Plus One
// https://leetcode.com/problems/plus-one/

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        println!("digits: {:?}", &digits);

        let len = digits.len();
        let mut digits = digits;
        digits[len - 1] += 1;

        for i in (0..len).rev() {
            if digits[i] > 9 {
                digits[i] = digits[i] % 10;
                if i == 0 {
                    digits.insert(0, 1);
                } else {
                    digits[i - 1] += 1;
                }
            }
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::plus_one::Solution;

    #[test]
    fn test_plus_one_1() {
        let digits = [1, 2, 3].to_vec();
        let output = [1, 2, 4].to_vec();
        assert_eq!(output, Solution::plus_one(digits));
    }

    #[test]
    fn test_plus_one_2() {
        let digits = [4, 3, 2, 1].to_vec();
        let output = [4, 3, 2, 2].to_vec();
        assert_eq!(output, Solution::plus_one(digits));
    }

    #[test]
    fn test_plus_one_3() {
        let digits = [9].to_vec();
        let output = [1, 0].to_vec();
        assert_eq!(output, Solution::plus_one(digits));
    }
}
