// 756. Pyramid Transition Matrix
// https://leetcode.com/problems/pyramid-transition-matrix/

struct Solution;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::pyramid_transition_matrix::Solution;

    #[test]
    fn test_pyramid_transition_1() {
        let bottom = "BCD".to_string();
        let allowed = ["BCC", "CDE", "CEA", "FFF"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(true, Solution::pyramid_transition(bottom, allowed));
    }

    #[test]
    fn test_pyramid_transition_2() {
        let bottom = "AAAA".to_string();
        let allowed = ["AAB", "AAC", "BCD", "BBE", "DEF"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(false, Solution::pyramid_transition(bottom, allowed));
    }
}
