// 3464. Maximize the Distance Between Points on a Square
// https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square/

struct Solution;

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let s = side as i64;
        let perimeter = 4 * s;
        let k = k as usize;
        let n = points.len();

        let mut pos: Vec<i64> = points
            .iter()
            .map(|p| {
                let (x, y) = (p[0] as i64, p[1] as i64);
                if y == 0 {
                    x
                } else if x == s {
                    s + y
                } else if y == s {
                    3 * s - x
                } else {
                    4 * s - y
                }
            })
            .collect();
        pos.sort_unstable();

        // Extended array for circular handling
        let ext: Vec<i64> = pos.iter().copied().chain(pos.iter().map(|&p| p + perimeter)).collect();
        let m = 2 * n;

        let can = |d: i64| -> bool {
            if d == 0 {
                return true;
            }
            // nxt[i] = first index j in ext where ext[j] >= ext[i] + d
            let mut nxt = vec![m; m];
            let mut j = 0usize;
            for i in 0..m {
                if j <= i {
                    j = i + 1;
                }
                while j < m && ext[j] < ext[i] + d {
                    j += 1;
                }
                nxt[i] = j;
            }

            // Binary lifting
            let log_k = 64 - (k as u64).leading_zeros() as usize; // ceil(log2(k)) + 1
            let mut jump = vec![vec![m; m]; log_k + 1];
            jump[0] = nxt;
            for t in 1..=log_k {
                for i in 0..m {
                    let mid = jump[t - 1][i];
                    jump[t][i] = if mid < m { jump[t - 1][mid] } else { m };
                }
            }

            let steps = k - 1;
            for i in 0..n {
                let mut cur = i;
                for t in 0..=log_k {
                    if steps & (1 << t) != 0 {
                        cur = jump[t][cur];
                        if cur >= m {
                            break;
                        }
                    }
                }
                if cur < m && ext[cur] <= pos[i] + perimeter - d {
                    return true;
                }
            }
            false
        };

        let mut lo: i64 = 0;
        let mut hi: i64 = perimeter / k as i64;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if can(mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_distance_between_points_on_a_square::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_distance_1() {
        let side = 2;
        let points = to_vec2d([[0, 2], [2, 0], [2, 2], [0, 0]]);
        let k = 4;
        assert_eq!(2, Solution::max_distance(side, points, k));
    }

    #[test]
    fn test_max_distance_2() {
        let side = 2;
        let points = to_vec2d([[0, 0], [1, 2], [2, 0], [2, 2], [2, 1]]);
        let k = 4;
        assert_eq!(1, Solution::max_distance(side, points, k));
    }

    #[test]
    fn test_max_distance_3() {
        let side = 2;
        let points = to_vec2d([[0, 0], [0, 1], [0, 2], [1, 2], [2, 0], [2, 2], [2, 1]]);
        let k = 5;
        assert_eq!(1, Solution::max_distance(side, points, k));
    }
}
