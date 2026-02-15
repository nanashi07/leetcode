// 67. Add Binary
// https://leetcode.com/problems/add-binary/

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        println!("a: {a}, b: {b}");

        let a_len = a.len();
        let b_len = b.len();
        let len = a_len.max(b_len);
        let mut next = 0;
        let mut s = String::new();

        for i in 0..len {
            let mut n = next;
            next = 0;

            if i < a_len && &a[a_len - i - 1..a_len - i] == "1" {
                n += 1;
            }

            if i < b_len && &b[b_len - i - 1..b_len - i] == "1" {
                n += 1;
            }

            if n > 1 {
                next = 1;
            }
            s.insert(0, if n % 2 == 1 { '1' } else { '0' });
        }

        if next > 0 {
            s.insert(0, '1');
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::add_binary::Solution;

    #[test]
    fn test_add_binary_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!("100".to_string(), Solution::add_binary(a, b));
    }

    #[test]
    fn test_add_binary_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!("10101".to_string(), Solution::add_binary(a, b));
    }
}
