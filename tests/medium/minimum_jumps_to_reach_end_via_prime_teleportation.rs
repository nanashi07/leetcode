// 3629. Minimum Jumps to Reach End via Prime Teleportation
// https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation/

struct Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};

        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // Collect all prime values that appear in nums
        let prime_values: HashSet<i32> = nums
            .iter()
            .copied()
            .filter(|&x| Self::is_prime(x))
            .collect();

        // For each prime p appearing as a value, group all indices j where p | nums[j]
        // Also track which groups each index belongs to
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut index_groups: Vec<Vec<i32>> = vec![vec![]; n];

        for (j, &val) in nums.iter().enumerate() {
            for p in Self::prime_factors(val) {
                if prime_values.contains(&p) {
                    groups.entry(p).or_default().push(j);
                    index_groups[j].push(p);
                }
            }
        }

        // BFS (Jump Game IV style with group clearing optimization)
        let mut visited = vec![false; n];
        let mut used_groups: HashSet<i32> = HashSet::new();
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back((0usize, 0i32));

        while let Some((i, jumps)) = queue.pop_front() {
            // Adjacent moves
            for ni in [i.wrapping_sub(1), i + 1] {
                if ni < n && !visited[ni] {
                    if ni == n - 1 {
                        return jumps + 1;
                    }
                    visited[ni] = true;
                    queue.push_back((ni, jumps + 1));
                }
            }
            // Teleport via prime groups
            for &p in &index_groups[i] {
                if used_groups.contains(&p) {
                    continue;
                }
                used_groups.insert(p);
                if let Some(group) = groups.get(&p) {
                    for &j in group {
                        if !visited[j] {
                            if j == n - 1 {
                                return jumps + 1;
                            }
                            visited[j] = true;
                            queue.push_back((j, jumps + 1));
                        }
                    }
                }
            }
        }

        -1
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n < 4 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn prime_factors(mut n: i32) -> Vec<i32> {
        let mut factors = vec![];
        if n <= 1 {
            return factors;
        }
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                factors.push(d);
                while n % d == 0 {
                    n /= d;
                }
            }
            d += 1;
        }
        if n > 1 {
            factors.push(n);
        }
        factors
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

    #[test]
    fn test_min_jumps_4() {
        let nums = [2, 7, 7].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }
}
