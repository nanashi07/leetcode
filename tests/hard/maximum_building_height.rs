// 1840. Maximum Building Height
// https://leetcode.com/problems/maximum-building-height/

struct Solution;

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut pts: Vec<(i64, i64)> = restrictions
            .iter()
            .map(|r| (r[0] as i64, r[1] as i64))
            .collect();
        pts.push((1, 0));
        pts.sort();
        if pts.last().unwrap().0 != n as i64 {
            pts.push((n as i64, n as i64 - 1));
        }

        let len = pts.len();
        for i in 1..len {
            pts[i].1 = pts[i].1.min(pts[i - 1].1 + (pts[i].0 - pts[i - 1].0));
        }
        for i in (0..len - 1).rev() {
            pts[i].1 = pts[i].1.min(pts[i + 1].1 + (pts[i + 1].0 - pts[i].0));
        }

        let mut ans: i64 = 0;
        for i in 1..len {
            let d = pts[i].0 - pts[i - 1].0;
            let h = (d + pts[i - 1].1 + pts[i].1) / 2;
            ans = ans.max(h);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_building_height::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_building_1() {
        let n = 5;
        let restrictions = to_vec2d([[2, 1], [4, 1]]);
        assert_eq!(2, Solution::max_building(n, restrictions));
    }

    #[test]
    fn test_max_building_2() {
        let n = 6;
        let restrictions = vec![];
        assert_eq!(5, Solution::max_building(n, restrictions));
    }

    #[test]
    fn test_max_building_3() {
        let n = 10;
        let restrictions = to_vec2d([[5, 3], [2, 5], [7, 4], [10, 3]]);
        assert_eq!(5, Solution::max_building(n, restrictions));
    }
}
