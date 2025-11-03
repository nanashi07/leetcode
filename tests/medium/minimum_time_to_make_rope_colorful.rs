// 1578. Minimum Time to Make Rope Colorful
// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/

struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_time_to_make_rope_colorful::Solution;

    #[test]
    fn test_min_cost_1() {
        let colors = "abaac".to_string();
        let needed_time = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(3, Solution::min_cost(colors, needed_time));
    }

    #[test]
    fn test_min_cost_2() {
        let colors = "abc".to_string();
        let needed_time = [1, 2, 3].to_vec();
        assert_eq!(0, Solution::min_cost(colors, needed_time));
    }

    #[test]
    fn test_min_cost_3() {
        let colors = "aabaa".to_string();
        let needed_time = [1, 2, 3, 4, 1].to_vec();
        assert_eq!(2, Solution::min_cost(colors, needed_time));
    }
}
