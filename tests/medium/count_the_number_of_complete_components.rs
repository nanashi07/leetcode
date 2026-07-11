// 2685. Count the Number of Complete Components
// https://leetcode.com/problems/count-the-number-of-complete-components/

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_complete_components::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_complete_components_1() {
        let n = 6;
        let edges = to_vec2d([[0, 1], [0, 2], [1, 2], [3, 4]]);
        assert_eq!(3, Solution::count_complete_components(n, edges));
    }

    #[test]
    fn test_count_complete_components_2() {
        let n = 6;
        let edges = to_vec2d([[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]);
        assert_eq!(1, Solution::count_complete_components(n, edges));
    }
}
