// 2211. Count Collisions on a Road
// https://leetcode.com/problems/count-collisions-on-a-road/

struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_collisions_on_a_road::Solution;

    #[test]
    fn test_count_collisions_1() {
        let directions = "RLRSLL".to_string();
        assert_eq!(5, Solution::count_collisions(directions));
    }

    #[test]
    fn test_count_collisions_2() {
        let directions = "LLRR".to_string();
        assert_eq!(0, Solution::count_collisions(directions));
    }
}
