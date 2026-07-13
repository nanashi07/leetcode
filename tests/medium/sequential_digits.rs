// 1291. Sequential Digits
// https://leetcode.com/problems/sequential-digits/

struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sequential_digits::Solution;

    #[test]
    fn test_sequential_digits_1() {
        let low = 100;
        let high = 300;
        assert_eq!([123, 234].to_vec(), Solution::sequential_digits(low, high));
    }

    #[test]
    fn test_sequential_digits_2() {
        let low = 1000;
        let high = 13000;
        assert_eq!(
            [1234, 2345, 3456, 4567, 5678, 6789, 12345].to_vec(),
            Solution::sequential_digits(low, high)
        );
    }
}
