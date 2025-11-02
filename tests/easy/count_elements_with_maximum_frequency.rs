// 3005. Count Elements With Maximum Frequency
// https://leetcode.com/problems/count-elements-with-maximum-frequency/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
            max = max.max(*map.get(&num).unwrap());
        }

        map.values().filter(|&v| v == &max).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_elements_with_maximum_frequency::Solution;

    #[test]
    fn test_max_frequency_elements_1() {
        let nums = [1, 2, 2, 3, 1, 4].to_vec();
        assert_eq!(4, Solution::max_frequency_elements(nums));
    }

    #[test]
    fn test_max_frequency_elements_2() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(5, Solution::max_frequency_elements(nums));
    }
}
