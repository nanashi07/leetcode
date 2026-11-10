// 3693. Climbing Stairs II
// https://leetcode.com/problems/climbing-stairs-ii/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        // dp[j] = cheapest total cost to stand on step `j`; step 0 is the start and is free.
        // Step `j` is reachable from steps j-1, j-2 or j-3, paying the landing cost plus d^2,
        // so each step only ever looks back three steps: a single forward pass suffices.
        let len = n as usize;
        let mut dp = vec![0i32; len + 1];

        for (idx, &cost) in costs.iter().enumerate() {
            let step = idx + 1; // `costs` is 1-indexed: costs[idx] belongs to step idx + 1
            let mut best = dp[step - 1] + 1;
            if step >= 2 {
                best = best.min(dp[step - 2] + 4);
            }
            if step >= 3 {
                best = best.min(dp[step - 3] + 9);
            }
            dp[step] = cost + best;
        }

        dp[len]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::climbing_stairs_ii::Solution;

    #[test]
    fn test_climb_stairs_1() {
        let n = 4;
        let costs = [1, 2, 3, 4].to_vec();
        assert_eq!(13, Solution::climb_stairs(n, costs));
    }

    #[test]
    fn test_climb_stairs_2() {
        let n = 4;
        let costs = [5, 1, 6, 2].to_vec();
        assert_eq!(11, Solution::climb_stairs(n, costs));
    }

    #[test]
    fn test_climb_stairs_3() {
        let n = 3;
        let costs = [9, 8, 3].to_vec();
        assert_eq!(12, Solution::climb_stairs(n, costs));
    }
}
