// # 2327. Number of People Aware of a Secret
// https://leetcode.com/problems/number-of-people-aware-of-a-secret/

struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        println!("n: {n}, delay: {delay}, forget: {forget}");

        const M: i32 = 10_i32.pow(9) + 7;

        // dp[i] = number of people who learned the secret on day i
        let mut dp = vec![0i64; (n + 1) as usize];
        dp[1] = 1; // On day 1, 1 person learns the secret

        for day in 2..=n {
            let day_idx = day as usize;

            // Calculate how many people can share the secret on this day
            // People who learned the secret from day max(1, day-forget+1) to day-delay
            // can share on this day (they haven't forgotten yet and delay has passed)
            for learn_day in 1.max(day - forget + 1)..=day - delay {
                if learn_day >= 1 && learn_day < day {
                    let learn_idx = learn_day as usize;
                    dp[day_idx] = (dp[day_idx] + dp[learn_idx]) % (M as i64);
                }
            }
        }

        // Count total people who know the secret on day n
        // These are people who learned from day max(1, n-forget+1) to day n
        let mut result = 0i64;
        for learn_day in 1.max(n - forget + 1)..=n {
            let learn_idx = learn_day as usize;
            result = (result + dp[learn_idx]) % (M as i64);
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_people_aware_of_a_secret::Solution;

    #[test]
    fn test_people_aware_of_secret_1() {
        let n = 6;
        let delay = 2;
        let forget = 4;
        assert_eq!(5, Solution::people_aware_of_secret(n, delay, forget));
    }

    #[test]
    fn test_people_aware_of_secret_2() {
        let n = 4;
        let delay = 1;
        let forget = 3;
        assert_eq!(6, Solution::people_aware_of_secret(n, delay, forget));
    }
}
