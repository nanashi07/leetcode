// 2839. Check if Strings Can be Made Equal With Operations I
// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i/

struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1;

        for i in 0..s1.len() - 2 {
            let j = i + 2;
            if &s1[i..=i] != &s2[i..=i] {
                if &s1[i..=i] != &s2[j..=j] {
                    return false;
                }
                let p1 = s1[i..=i].to_string();
                s1.replace_range(i..=i, &s2[j..=j]);
                s1.replace_range(j..=j, &p1);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_strings_can_be_made_equal_with_operations_i::Solution;

    #[test]
    fn test_can_be_equal_1() {
        let s1 = "abcd".to_string();
        let s2 = "cdab".to_string();
        assert_eq!(true, Solution::can_be_equal(s1, s2));
    }

    #[test]
    fn test_can_be_equal_2() {
        let s1 = "abcd".to_string();
        let s2 = "dacb".to_string();
        assert_eq!(false, Solution::can_be_equal(s1, s2));
    }
}
