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

    #[test]
    fn test_next_greatest_letter_1() {
        let letters = ["c", "f", "j"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let target = 'a';
        assert_eq!('c', Solution::next_greatest_letter(letters, target));
    }

    #[test]
    fn test_next_greatest_letter_2() {
        let letters = ["c", "f", "j"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let target = 'c';
        assert_eq!('f', Solution::next_greatest_letter(letters, target));
    }

    #[test]
    fn test_next_greatest_letter_3() {
        let letters = ["x", "x", "y", "y"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let target = 'z';
        assert_eq!('x', Solution::next_greatest_letter(letters, target));
    }
}
