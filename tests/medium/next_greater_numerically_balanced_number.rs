// # 2048. Next Greater Numerically Balanced Number
// https://leetcode.com/problems/next-greater-numerically-balanced-number/

struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        // Start checking from n+1
        let mut candidate = n + 1;

        // Upper bound: 7 digits max (since we can't have digit 8 appear 8 times in a 7-digit number)
        // Actually, max is 1224444 (7 digits)
        while candidate <= 10_000_000 {
            if Self::is_balanced(candidate) {
                return candidate;
            }
            candidate += 1;
        }

        candidate
    }

    fn is_balanced(num: i32) -> bool {
        // Count frequency of each digit
        let mut freq = [0; 10];
        let mut temp = num;

        while temp > 0 {
            let digit = (temp % 10) as usize;
            freq[digit] += 1;
            temp /= 10;
        }

        // Check if each digit appears exactly as many times as its value
        for digit in 0..10 {
            if freq[digit] != 0 && freq[digit] != digit {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::next_greater_numerically_balanced_number::Solution;

    #[test]
    fn test_next_beautiful_number_1() {
        let n = 1;
        assert_eq!(22, Solution::next_beautiful_number(n));
    }

    #[test]
    fn test_next_beautiful_number_2() {
        let n = 1000;
        assert_eq!(1333, Solution::next_beautiful_number(n));
    }

    #[test]
    fn test_next_beautiful_number_3() {
        let n = 3000;
        assert_eq!(3133, Solution::next_beautiful_number(n));
    }
}
