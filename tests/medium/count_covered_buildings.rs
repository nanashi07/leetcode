// 3531. Count Covered Buildings
// https://leetcode.com/problems/count-covered-buildings/

struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_covered_buildings::Solution;

    #[test]
    fn test_count_covered_buildings_1() {
        let n = 3;
        let buildings = [[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_covered_buildings(n, buildings));
    }

    #[test]
    fn test_count_covered_buildings_2() {
        let n = 3;
        let buildings = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::count_covered_buildings(n, buildings));
    }

    #[test]
    fn test_count_covered_buildings_3() {
        let n = 5;
        let buildings = [[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_covered_buildings(n, buildings));
    }
}
