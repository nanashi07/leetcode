// 3620. Network Recovery Pathways
// https://leetcode.com/problems/network-recovery-pathways/

struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::network_recovery_pathways::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_max_path_score_1() {
        let edges = to_vec2d([[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]]);
        let online = [true, true, true, true].to_vec();
        let k = 10;
        assert_eq!(3, Solution::find_max_path_score(edges, online, k));
    }

    #[test]
    fn test_find_max_path_score_2() {
        let edges = to_vec2d([
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ]);
        let online = [true, true, true, false, true].to_vec();
        let k = 12;
        assert_eq!(6, Solution::find_max_path_score(edges, online, k));
    }
}
