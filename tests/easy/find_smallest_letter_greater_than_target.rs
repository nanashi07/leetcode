// 744. Find Smallest Letter Greater Than Target
// https://leetcode.com/problems/find-smallest-letter-greater-than-target/

struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        println!("letters: {letters:?}, target: {target}");

        for c in &letters {
            if *c > target {
                return *c;
            }
        }

        letters[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_smallest_letter_greater_than_target::Solution;
    use crate::shared::vec2d::to_char_vec;

    #[test]
    fn test_next_greatest_letter_1() {
        let letters = to_char_vec(["c", "f", "j"]);
        let target = 'a';
        assert_eq!('c', Solution::next_greatest_letter(letters, target));
    }

    #[test]
    fn test_next_greatest_letter_2() {
        let letters = to_char_vec(["c", "f", "j"]);
        let target = 'c';
        assert_eq!('f', Solution::next_greatest_letter(letters, target));
    }

    #[test]
    fn test_next_greatest_letter_3() {
        let letters = to_char_vec(["x", "x", "y", "y"]);
        let target = 'z';
        assert_eq!('x', Solution::next_greatest_letter(letters, target));
    }
}
