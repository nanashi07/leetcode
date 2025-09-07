// # 1304. Find N Unique Integers Sum up to Zero
// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n % 2 == 0 {
            ((n / -2)..=(n / 2))
                .into_iter()
                .filter(|n| *n != 0)
                .collect::<Vec<i32>>()
        } else {
            ((n / -2)..=(n / 2)).into_iter().collect::<Vec<i32>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_n_unique_integers_sum_up_to_zero::Solution;

    #[test]
    fn test_sum_zero_1() {
        let n = 5;
        assert_eq!(0, Solution::sum_zero(n).iter().sum());
    }

    #[test]
    fn test_sum_zero_2() {
        let n = 3;
        assert_eq!(0, Solution::sum_zero(n).iter().sum());
    }

    #[test]
    fn test_sum_zero_3() {
        let n = 1;
        assert_eq!(0, Solution::sum_zero(n).iter().sum());
    }
}
