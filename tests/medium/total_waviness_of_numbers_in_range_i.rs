// 3751. Total Waviness of Numbers in Range I
// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i/

struct Solution;

type Memo = Vec<Vec<Vec<Vec<Vec<Option<(i64, i64)>>>>>>;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (Self::total_up_to(num2) - Self::total_up_to(num1 - 1)) as i32
    }

    fn is_wavy(a: i32, b: i32, c: i32) -> i64 {
        ((a < b && b > c) || (a > b && b < c)) as i64
    }

    fn solve(
        pos: usize,
        tight: usize,
        started: usize,
        prev1: usize,
        prev2: usize,
        digits: &[u8],
        memo: &mut Memo,
    ) -> (i64, i64) {
        if pos == digits.len() {
            return (1, 0);
        }
        if let Some(result) = memo[pos][tight][started][prev1][prev2] {
            return result;
        }

        let limit = if tight == 1 { digits[pos] } else { 9 } as usize;
        let mut count = 0i64;
        let mut total = 0i64;

        for digit in 0..=limit {
            let next_tight = usize::from(tight == 1 && digit == limit);

            if started == 0 && digit == 0 {
                let (sub_count, sub_total) =
                    Self::solve(pos + 1, next_tight, 0, 10, 10, digits, memo);
                count += sub_count;
                total += sub_total;
                continue;
            }

            if started == 0 {
                let (sub_count, sub_total) =
                    Self::solve(pos + 1, next_tight, 1, digit, 10, digits, memo);
                count += sub_count;
                total += sub_total;
                continue;
            }

            let extra = if prev2 == 10 {
                0
            } else {
                Self::is_wavy(prev2 as i32, prev1 as i32, digit as i32)
            };
            let (sub_count, sub_total) =
                Self::solve(pos + 1, next_tight, 1, digit, prev1, digits, memo);
            count += sub_count;
            total += sub_total + extra * sub_count;
        }

        let result = (count, total);
        memo[pos][tight][started][prev1][prev2] = Some(result);
        result
    }

    fn total_up_to(n: i32) -> i64 {
        if n <= 0 {
            return 0;
        }

        let digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
        let len = digits.len();
        let mut memo = vec![vec![vec![vec![vec![None; 11]; 11]; 2]; 2]; len];
        Self::solve(0, 1, 0, 10, 10, &digits, &mut memo).1
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::total_waviness_of_numbers_in_range_i::Solution;

    #[test]
    fn test_total_waviness_1() {
        let num1 = 120;
        let num2 = 130;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_2() {
        let num1 = 198;
        let num2 = 202;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_3() {
        let num1 = 4848;
        let num2 = 4848;
        assert_eq!(2, Solution::total_waviness(num1, num2));
    }
}
