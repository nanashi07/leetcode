// # 1716. Calculate Money in Leetcode Bank
// https://leetcode.com/problems/calculate-money-in-leetcode-bank/

struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        println!("n: {n}");

        let mut amount = 0;
        for i in 0..n {
            let week = i / 7;
            let day = i % 7;
            amount += week + day + 1;
        }

        amount
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::calculate_money_in_leetcode_bank::Solution;

    #[test]
    fn test_total_money_1() {
        let n = 4;
        assert_eq!(10, Solution::total_money(n));
    }

    #[test]
    fn test_total_money_2() {
        let n = 10;
        assert_eq!(37, Solution::total_money(n));
    }

    #[test]
    fn test_total_money_3() {
        let n = 20;
        assert_eq!(96, Solution::total_money(n));
    }
}
