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

        let max_val = nums.iter().copied().max().unwrap_or(0).max(0) as usize;
        let spf = Self::smallest_prime_factors(max_val);

        // Prime values that appear in nums are valid teleport "keys".
        let prime_values: HashSet<i32> = nums
            .iter()
            .copied()
            .filter(|&x| Self::is_prime(x))
            .collect();

        // For each prime p in nums, store all indices j where p divides nums[j].
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
        for (j, &val) in nums.iter().enumerate() {
            for p in Self::prime_factors(val, &spf) {
                if prime_values.contains(&p) {
                    groups.entry(p).or_default().push(j);
                }
            }
        }

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

            // Teleport is allowed only when current value itself is prime.
            let p = nums[i];
            if Self::is_prime(p) && used_groups.insert(p) {
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

    fn smallest_prime_factors(limit: usize) -> Vec<usize> {
        let mut spf = vec![0usize; limit + 1];
        for i in 2..=limit {
            if spf[i] == 0 {
                spf[i] = i;
                if i <= limit / i {
                    let mut j = i * i;
                    while j <= limit {
                        if spf[j] == 0 {
                            spf[j] = i;
                        }
                        j += i;
                    }
                }
            }
        }
        spf
    }

    fn prime_factors(mut n: i32, spf: &[usize]) -> Vec<i32> {
        let mut factors = vec![];
        if n <= 1 {
            return factors;
        }
        while n > 1 {
            let p = spf[n as usize] as i32;
            factors.push(p);
            while n % p == 0 {
                n /= p;
            }
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

    #[test]
    fn test_min_jumps_5() {
        let nums = [4, 5, 2].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }
}
