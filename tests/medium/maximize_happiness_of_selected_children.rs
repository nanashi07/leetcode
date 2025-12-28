// 3075. Maximize Happiness of Selected Children
// https://leetcode.com/problems/maximize-happiness-of-selected-children/

struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        // Sort in descending order to select the happiest children first
        happiness.sort_by(|a, b| b.cmp(a));

        let mut total_happiness = 0i64;

        // Select k children
        for i in 0..k as usize {
            // Each child's happiness decreases by the number of previously selected children
            let adjusted_happiness = happiness[i] - i as i32;
            // Happiness cannot be negative
            if adjusted_happiness > 0 {
                total_happiness += adjusted_happiness as i64;
            }
        }

        total_happiness
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximize_happiness_of_selected_children::Solution;

    #[test]
    fn test_maximum_happiness_sum_1() {
        let happiness = [1, 2, 3].to_vec();
        let k = 2;
        assert_eq!(4, Solution::maximum_happiness_sum(happiness, k));
    }

    #[test]
    fn test_maximum_happiness_sum_2() {
        let happiness = [1, 1, 1, 1].to_vec();
        let k = 2;
        assert_eq!(1, Solution::maximum_happiness_sum(happiness, k));
    }

    #[test]
    fn test_maximum_happiness_sum_3() {
        let happiness = [2, 3, 4, 5].to_vec();
        let k = 1;
        assert_eq!(5, Solution::maximum_happiness_sum(happiness, k));
    }
}
