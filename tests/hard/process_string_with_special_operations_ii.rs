// 3614. Process String with Special Operations II
// https://leetcode.com/problems/process-string-with-special-operations-ii/

struct Solution;

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let bytes = s.as_bytes();
        let n = bytes.len();
        const CAP: i64 = 4_000_000_000_000_000_000;

        let mut lens = vec![0i64; n + 1];
        for i in 0..n {
            lens[i + 1] = match bytes[i] {
                b'*' => (lens[i] - 1).max(0),
                b'#' => lens[i].saturating_mul(2).min(CAP),
                b'%' => lens[i],
                _ => lens[i].saturating_add(1).min(CAP),
            };
        }

        if k >= lens[n] {
            return '.';
        }

        let mut k = k;
        for i in (0..n).rev() {
            match bytes[i] {
                b'*' => {
                    if k == lens[i] {
                        return '.';
                    }
                }
                b'#' => {
                    if lens[i] == 0 {
                        continue;
                    }
                    k %= lens[i];
                }
                b'%' => {
                    k = lens[i] - 1 - k;
                }
                c => {
                    if k == lens[i] {
                        return c as char;
                    }
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::process_string_with_special_operations_ii::Solution;

    #[test]
    fn test_process_str_1() {
        let s = "a#b%*".to_string();
        let k = 1;
        assert_eq!('a', Solution::process_str(s, k));
    }

    #[test]
    fn test_process_str_2() {
        let s = "cd%#*#".to_string();
        let k = 3;
        assert_eq!('d', Solution::process_str(s, k));
    }

    #[test]
    fn test_process_str_3() {
        let s = "z*#".to_string();
        let k = 0;
        assert_eq!('.', Solution::process_str(s, k));
    }

    #[test]
    fn test_process_str_4() {
        let s = "#jief%k".to_string();
        let k = 0;
        assert_eq!('f', Solution::process_str(s, k));
    }
}
