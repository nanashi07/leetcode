// 1189. Maximum Number of Balloons
// https://leetcode.com/problems/maximum-number-of-balloons/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut balloon_map: HashMap<char, i32> = HashMap::new();
        for c in "balloon".chars() {
            *balloon_map.entry(c).or_insert(0) += 1;
        }
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in text.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut min: f64 = i32::MAX as f64;
        for (k, v) in &balloon_map {
            if map.contains_key(k) {
                let t = map[k];
                min = min.min(t as f64 / *v as f64);
            } else {
                return 0;
            }
        }
        min as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_number_of_balloons::Solution;

    #[test]
    fn test_max_number_of_balloons_1() {
        let text = "nlaebolko".to_string();
        assert_eq!(1, Solution::max_number_of_balloons(text));
    }

    #[test]
    fn test_max_number_of_balloons_2() {
        let text = "loonbalxballpoon".to_string();
        assert_eq!(2, Solution::max_number_of_balloons(text));
    }

    #[test]
    fn test_max_number_of_balloons_3() {
        let text = "leetcode".to_string();
        assert_eq!(0, Solution::max_number_of_balloons(text));
    }
}
