// 3867. Sum of GCD of Formed Pairs
// https://leetcode.com/problems/sum-of-gcd-of-formed-pairs/

struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }

        let n = nums.len();
        let mut prefix_gcd = Vec::with_capacity(n);
        let mut mx = 0;
        for &x in &nums {
            mx = mx.max(x);
            prefix_gcd.push(gcd(x, mx));
        }

        prefix_gcd.sort_unstable();

        let mut ans: i64 = 0;
        for i in 0..n / 2 {
            ans += gcd(prefix_gcd[i], prefix_gcd[n - 1 - i]) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sum_of_gcd_of_formed_pairs::Solution;

    #[test]
    fn test_gcd_sum_1() {
        let nums = [2, 6, 4].to_vec();
        assert_eq!(2, Solution::gcd_sum(nums));
    }

    #[test]
    fn test_gcd_sum_2() {
        let nums = [3, 6, 2, 8].to_vec();
        assert_eq!(5, Solution::gcd_sum(nums));
    }
}
