// 3629. Minimum Jumps to Reach End via Prime Teleportation
// https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation/

struct Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        visited[0] = true;
        if 0 == n - 1 {
            return 0;
        }
        queue.push_back((0usize, 0i32));

        while let Some((i, jumps)) = queue.pop_front() {
            let mut nexts = vec![i + 1];
            if Self::is_prime(nums[i]) {
                nexts.push(i + nums[i] as usize);
            }
            for ni in nexts {
                if ni < n && !visited[ni] {
                    if ni == n - 1 {
                        return jumps + 1;
                    }
                    visited[ni] = true;
                    queue.push_back((ni, jumps + 1));
                }
            }
        }

        -1
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 { return false; }
        if n < 4 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_jumps_to_reach_end_via_prime_teleportation::Solution;

    #[test]
    fn test_min_jumps_1() {
        let nums = [1, 2, 4, 6].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }

    #[test]
    fn test_min_jumps_2() {
        let nums = [2, 3, 4, 7, 9].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }

    #[test]
    fn test_min_jumps_3() {
        let nums = [4, 6, 5, 8].to_vec();
        assert_eq!(3, Solution::min_jumps(nums));
    }
}
