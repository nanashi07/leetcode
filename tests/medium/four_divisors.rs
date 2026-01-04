// 1390. Four Divisors
// https://leetcode.com/problems/four-divisors/

struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut total = 0;

        for &num in &nums {
            let mut divisors = Vec::new();
            let sqrt = (num as f64).sqrt() as i32;

            // Find all divisors
            for i in 1..=sqrt {
                if num % i == 0 {
                    divisors.push(i);
                    if i != num / i {
                        divisors.push(num / i);
                    }
                }

                // Early exit if more than 4 divisors
                if divisors.len() > 4 {
                    break;
                }
            }

            // If exactly 4 divisors, add their sum
            if divisors.len() == 4 {
                total += divisors.iter().sum::<i32>();
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::four_divisors::Solution;

    #[test]
    fn test_sum_four_divisors_1() {
        let nums = [21, 4, 7].to_vec();
        assert_eq!(32, Solution::sum_four_divisors(nums));
    }

    #[test]
    fn test_sum_four_divisors_2() {
        let nums = [21, 21].to_vec();
        assert_eq!(64, Solution::sum_four_divisors(nums));
    }

    #[test]
    fn test_sum_four_divisors_3() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(0, Solution::sum_four_divisors(nums));
    }
}
