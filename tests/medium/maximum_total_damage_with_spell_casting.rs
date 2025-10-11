// # 3186. Maximum Total Damage With Spell Casting
// https://leetcode.com/problems/maximum-total-damage-with-spell-casting/

struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        use std::collections::HashMap;

        if power.is_empty() {
            return 0;
        }

        // Count frequency of each power value
        let mut freq: HashMap<i32, i64> = HashMap::new();
        for &p in &power {
            *freq.entry(p).or_insert(0) += 1;
        }

        // Get sorted unique power values
        let mut unique: Vec<i32> = freq.keys().copied().collect();
        unique.sort_unstable();

        let n = unique.len();
        if n == 0 {
            return 0;
        }

        // dp[i] = max damage considering unique[0..=i]
        let mut dp = vec![0i64; n];

        // Base case: take the first unique power
        dp[0] = unique[0] as i64 * freq[&unique[0]];

        for i in 1..n {
            let curr_power = unique[i];
            let curr_damage = curr_power as i64 * freq[&curr_power];

            // Option 1: don't take current power
            let mut max_damage = dp[i - 1];

            // Option 2: take current power
            // Find the last power value that can be taken with current
            // (must be < curr_power - 2, i.e., <= curr_power - 3)
            let mut take_damage = curr_damage;

            // Binary search for the rightmost index where unique[j] < curr_power - 2
            let target = curr_power - 2;
            let mut left = 0;
            let mut right = i;
            let mut last_valid = -1i32;

            while left < right {
                let mid = left + (right - left) / 2;
                if unique[mid] < target {
                    last_valid = mid as i32;
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if last_valid >= 0 {
                take_damage += dp[last_valid as usize];
            }

            max_damage = max_damage.max(take_damage);
            dp[i] = max_damage;
        }

        dp[n - 1]
    }

    // Time Limit Exceeded
    pub fn _maximum_total_damage(power: Vec<i32>) -> i64 {
        println!("power: {:?}", &power);

        let mut power = power;
        power.sort_unstable();
        power.reverse();

        Self::_next(&power[..], 0)
    }

    fn _next(remain: &[i32], total_damage: i64) -> i64 {
        if remain.is_empty() {
            return total_damage;
        }

        if remain.len() == 1 {
            return total_damage + remain[0] as i64;
        }

        let spell = remain[0];
        let range = remain
            .iter()
            .take_while(|&s| *s >= spell - 2)
            .map(|&s| s)
            .collect::<Vec<i32>>();

        let damage = range
            .iter()
            .filter(|&s| *s == spell)
            .map(|&s| s as i64)
            .sum::<i64>();

        // choose
        let chosen = Self::_next(&remain[range.len()..], total_damage + damage);
        // ignore to next
        let ignored = Self::_next(&remain[1..], total_damage);

        chosen.max(ignored)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_total_damage_with_spell_casting::Solution;

    #[test]
    fn test_maximum_total_damage_1() {
        let power = [1, 1, 3, 4].to_vec();
        assert_eq!(6, Solution::maximum_total_damage(power));
    }

    #[test]
    fn test_maximum_total_damage_2() {
        let power = [7, 1, 6, 6].to_vec();
        assert_eq!(13, Solution::maximum_total_damage(power));
    }

    #[test]
    fn test_maximum_total_damage_3() {
        let power = [
            5, 58, 45, 54, 60, 6, 34, 26, 3, 64, 47, 58, 13, 31, 41, 32, 49, 10, 51, 27, 12, 24,
            37, 15, 11, 29, 6, 41, 10, 61, 17, 6, 23, 36, 63, 58, 50, 64, 55, 52, 46, 13, 33, 64,
            27, 41, 65, 27, 11, 27, 59, 53, 60, 37, 66, 10, 28, 32, 38, 26, 9, 45, 55, 9, 48, 22,
            22, 61, 62, 8, 41, 14, 23, 61, 40, 40, 5, 42, 60, 4, 55, 50, 30, 3, 58, 33, 27, 25, 6,
            32, 8, 33, 16, 34, 20, 14, 7, 19, 22,
        ]
        .to_vec();
        assert_eq!(1652, Solution::maximum_total_damage(power));
    }
}
