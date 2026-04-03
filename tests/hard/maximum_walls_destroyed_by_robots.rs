// 3661. Maximum Walls Destroyed by Robots
// https://leetcode.com/problems/maximum-walls-destroyed-by-robots/

struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let mut robots_dist: Vec<(i64, i64)> = robots
            .iter()
            .zip(distance.iter())
            .map(|(&r, &d)| (r as i64, d as i64))
            .collect();
        robots_dist.sort_unstable_by_key(|&(r, _)| r);

        // suffix_min[i] = min(robots_dist[j].0 - robots_dist[j].1) for j in [i, n)
        // A robot at position r with distance d covers walls in [r-d, r]
        // A wall w is reachable if there exists a robot with pos >= w and pos-dist <= w
        let mut suffix_min = vec![i64::MAX; n + 1];
        for i in (0..n).rev() {
            let (r, d) = robots_dist[i];
            suffix_min[i] = suffix_min[i + 1].min(r - d);
        }

        let mut count = 0i32;
        for &w in &walls {
            let w = w as i64;
            // Find first robot index with position >= w
            let j = robots_dist.partition_point(|&(r, _)| r < w);
            // Check if any robot from index j onward can reach w (pos-dist <= w)
            if suffix_min[j] <= w {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_walls_destroyed_by_robots::Solution;

    #[test]
    fn test_max_walls_1() {
        let robots = [4].to_vec();
        let distance = [3].to_vec();
        let walls = [1, 10].to_vec();
        assert_eq!(1, Solution::max_walls(robots, distance, walls));
    }

    #[test]
    fn test_max_walls_2() {
        let robots = [10, 2].to_vec();
        let distance = [5, 1].to_vec();
        let walls = [5, 2, 7].to_vec();
        assert_eq!(3, Solution::max_walls(robots, distance, walls));
    }

    #[test]
    fn test_max_walls_3() {
        let robots = [1, 2].to_vec();
        let distance = [100, 1].to_vec();
        let walls = [10].to_vec();
        assert_eq!(0, Solution::max_walls(robots, distance, walls));
    }
}
