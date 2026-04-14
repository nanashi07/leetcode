// 2463. Minimum Total Distance Traveled
// https://leetcode.com/problems/minimum-total-distance-traveled/

struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();

        // Flatten factories: each factory position repeated by its limit
        let facts: Vec<i64> = factory
            .iter()
            .flat_map(|f| std::iter::repeat(f[0] as i64).take(f[1] as usize))
            .collect();

        let n = robot.len();
        let m = facts.len();

        // dp[j] = min cost to assign robots[0..i] using flattened factories[0..j]
        let mut dp = vec![i64::MAX; m + 1];
        dp[0] = 0; // 0 robots assigned, 0 factories used

        for i in 1..=n {
            // Traverse j from right to left so we don't overwrite values we still need
            let mut new_dp = vec![i64::MAX; m + 1];
            for j in i..=m {
                // Option 1: skip factory j (don't assign robot i to factory j)
                new_dp[j] = new_dp[j - 1];
                // Option 2: assign robot i to factory j
                if dp[j - 1] != i64::MAX {
                    let cost = dp[j - 1] + (robot[i - 1] as i64 - facts[j - 1]).abs();
                    new_dp[j] = new_dp[j].min(cost);
                }
            }
            dp = new_dp;
        }

        dp[m]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_total_distance_traveled::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_minimum_total_distance_1() {
        let robot = [0, 4, 6].to_vec();
        let factory = to_vec2d([[2, 2], [6, 2]]);
        assert_eq!(4, Solution::minimum_total_distance(robot, factory));
    }

    #[test]
    fn test_minimum_total_distance_2() {
        let robot = [1, -1].to_vec();
        let factory = to_vec2d([[-2, 1], [2, 1]]);
        assert_eq!(2, Solution::minimum_total_distance(robot, factory));
    }
}
