// # 3541. Find Most Frequent Vowel and Consonant
// https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/

struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_most_frequent_vowel_and_consonant::Solution;

    #[test]
    fn test_max_freq_sum_1() {
        let s = "successes".to_owned();
        assert_eq!(6, Solution::max_freq_sum(s));
    }

    #[test]
    fn test_max_freq_sum_2() {
        let s = "aeiaeia".to_owned();
        assert_eq!(3, Solution::max_freq_sum(s));
    }
}
