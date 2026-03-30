// 2840. Check if Strings Can be Made Equal With Operations II
// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/

struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut even = [0i32; 26];
        let mut odd = [0i32; 26];
        for (i, (a, b)) in s1.bytes().zip(s2.bytes()).enumerate() {
            let (a, b) = ((a - b'a') as usize, (b - b'a') as usize);
            if i % 2 == 0 {
                even[a] += 1;
                even[b] -= 1;
            } else {
                odd[a] += 1;
                odd[b] -= 1;
            }
        }
        even.iter().all(|&x| x == 0) && odd.iter().all(|&x| x == 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::check_if_strings_can_be_made_equal_with_operations_ii::Solution;

    #[test]
    fn test_check_strings_1() {
        let s1 = "abcdba".to_string();
        let s2 = "cabdab".to_string();
        assert_eq!(true, Solution::check_strings(s1, s2));
    }

    #[test]
    fn test_check_strings_2() {
        let s1 = "abe".to_string();
        let s2 = "bea".to_string();
        assert_eq!(false, Solution::check_strings(s1, s2));
    }
}
