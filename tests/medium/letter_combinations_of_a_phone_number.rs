// 17. Letter Combinations of a Phone Number
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut r: Vec<String> = Vec::new();
        let alphabet = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        let len = digits.len();
        let group_index: Vec<usize> = digits.chars().map(|c| (c as usize) - 50).collect();
        let mut item_index = vec![0_usize; group_index.len()];

        while len > 0 {
            let mut s = String::new();
            for (ix, g) in group_index.iter().enumerate() {
                let g = *g;
                let i = item_index[ix];
                s.push_str(&alphabet[g][i..=i]);
            }

            r.push(s);

            let mut len = len - 1;
            item_index[len] = item_index[len] + 1;
            loop {
                if item_index[len] < alphabet[group_index[len]].len() {
                    break;
                } else {
                    if len == 0 {
                        return r;
                    } else {
                        item_index[len] = 0;
                        item_index[len - 1] = item_index[len - 1] + 1;
                    }
                }
                if len > 0 {
                    len = len - 1;
                }
            }
        }

        r
    }
}

#[test]
fn test_letter_combinations() {
    let digits = "23";
    let result = Solution::letter_combinations(digits.to_owned());
    assert_eq!(
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        result,
        "digits: {}",
        digits
    );

    let digits = "";
    let result = Solution::letter_combinations(digits.to_owned());
    let expected: Vec<String> = Vec::new();
    assert_eq!(expected, result, "digits: {}", digits);

    let digits = "2";
    let result = Solution::letter_combinations(digits.to_owned());
    assert_eq!(vec!["a", "b", "c"], result, "digits: {}", digits);

    let digits = "3";
    let result = Solution::letter_combinations(digits.to_owned());
    assert_eq!(vec!["d", "e", "f"], result, "digits: {}", digits);

    let digits = "7";
    let result = Solution::letter_combinations(digits.to_owned());
    assert_eq!(vec!["p", "q", "r", "s"], result, "digits: {}", digits);
}
