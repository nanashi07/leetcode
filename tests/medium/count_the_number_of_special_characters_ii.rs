// 3121. Count the Number of Special Characters II
// https://leetcode.com/problems/count-the-number-of-special-characters-ii/

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut last_lower = [-1_i32; 26];
        let mut first_upper = [i32::MAX; 26];

        for (i, &b) in word.as_bytes().iter().enumerate() {
            let pos = i as i32;
            if b.is_ascii_lowercase() {
                let idx = (b - b'a') as usize;
                last_lower[idx] = pos;
            } else {
                let idx = (b - b'A') as usize;
                first_upper[idx] = first_upper[idx].min(pos);
            }
        }

        let mut count = 0;
        for i in 0..26 {
            if last_lower[i] != -1 && first_upper[i] != i32::MAX && last_lower[i] < first_upper[i] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_special_characters_ii::Solution;

    #[test]
    fn test_number_of_special_chars_1() {
        let word = "aaAbcBC".to_string();
        assert_eq!(3, Solution::number_of_special_chars(word));
    }

    #[test]
    fn test_number_of_special_chars_2() {
        let word = "abc".to_string();
        assert_eq!(0, Solution::number_of_special_chars(word));
    }

    #[test]
    fn test_number_of_special_chars_3() {
        let word = "AbBCab".to_string();
        assert_eq!(0, Solution::number_of_special_chars(word));
    }
}
