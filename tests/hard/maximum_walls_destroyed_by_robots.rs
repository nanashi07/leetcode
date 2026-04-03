// 3661. Maximum Walls Destroyed by Robots
// https://leetcode.com/problems/maximum-walls-destroyed-by-robots/

struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();

        // Sort robots by position, pairing each with its distance
        let mut rd: Vec<(i64, i64)> = robots
            .iter()
            .zip(distance.iter())
            .map(|(&r, &d)| (r as i64, d as i64))
            .collect();
        rd.sort_unstable_by_key(|&(r, _)| r);

        // Sort walls for binary-search range counting
        let mut walls_sorted: Vec<i64> = walls.iter().map(|&w| w as i64).collect();
        walls_sorted.sort_unstable();

        // Count walls in the inclusive range [lo, hi]
        let count_in = |lo: i64, hi: i64| -> i64 {
            if lo > hi {
                return 0;
            }
            let l = walls_sorted.partition_point(|&w| w < lo);
            let r = walls_sorted.partition_point(|&w| w <= hi);
            (r - l) as i64
        };

        // Effective ranges per robot (blocked by adjacent robots):
        //   left range : [max(r-d, prev+1), r]
        //   right range: [r, min(r+d, next-1)]
        let left_lo = |i: usize| -> i64 {
            let lo = rd[i].0 - rd[i].1;
            if i > 0 { lo.max(rd[i - 1].0 + 1) } else { lo }
        };
        let right_hi = |i: usize| -> i64 {
            let hi = rd[i].0 + rd[i].1;
            if i + 1 < n { hi.min(rd[i + 1].0 - 1) } else { hi }
        };

        // DP: dp_l = best count when robot i goes LEFT
        //     dp_r = best count when robot i goes RIGHT
        //
        // Transitions (only adjacent-robot overlap matters):
        //   dp_l[i] = max(
        //       dp_l[i-1] + count([L[i], r_i]),                     // prev=L: no overlap
        //       dp_r[i-1] + count([max(L[i], R[i-1]+1), r_i])       // prev=R: trim overlap
        //   )
        //   dp_r[i] = max(dp_l[i-1], dp_r[i-1]) + count([r_i, R[i]])  // never overlaps prev
        let r0 = rd[0].0;
        let mut dp_l = count_in(left_lo(0), r0);
        let mut dp_r = count_in(r0, right_hi(0));

        for i in 1..n {
            let ri = rd[i].0;
            let li = left_lo(i);
            let ri_hi = right_hi(i);
            let prev_r_hi = right_hi(i - 1); // R[i-1]

            let new_dp_l = (dp_l + count_in(li, ri))
                .max(dp_r + count_in(li.max(prev_r_hi + 1), ri));
            let new_dp_r = dp_l.max(dp_r) + count_in(ri, ri_hi);

            dp_l = new_dp_l;
            dp_r = new_dp_r;
        }

        dp_l.max(dp_r) as i32
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

    #[test]
    fn test_max_walls_4() {
        let robots = [17, 59, 32, 11, 72, 18].to_vec();
        let distance = [5, 7, 6, 5, 2, 10].to_vec();
        let walls = [
            17, 25, 33, 29, 54, 53, 18, 35, 39, 37, 20, 14, 34, 13, 16, 58, 22, 51, 56, 27, 10, 15,
            12, 23, 45, 43, 21, 2, 42, 7, 32, 40, 8, 9, 1, 5, 55, 30, 38, 4, 3, 31, 36, 41, 57, 28,
            11, 49, 26, 19, 50, 52, 6, 47, 46, 44, 24, 48,
        ]
        .to_vec();
        assert_eq!(37, Solution::max_walls(robots, distance, walls));
    }
}
