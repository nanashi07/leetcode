// # 3541. Find Most Frequent Vowel and Consonant
// https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        println!("s: {}", &s);

        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut c1 = 0;
        let mut c2 = 0;

        let counter = s.chars().into_iter().fold(HashMap::new(), |mut a, b| {
            *a.entry(b).or_insert(0) += 1;
            a
        });

        for (k, v) in counter {
            if vowels.contains(&k) {
                if c1 < v {
                    c1 = v;
                }
            } else {
                if c2 < v {
                    c2 = v;
                }
            }
        }

        c1 + c2
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
