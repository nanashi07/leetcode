// 1415. The k-th Lexicographical String of All Happy Strings of Length n
// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/

struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        if n <= 0 || k <= 0 {
            return String::new();
        }

        let n = n as usize;
        let mut k = k as u64 - 1;
        let total = 3_u64 << (n - 1);

        if k >= total {
            return String::new();
        }

        let letters = [b'a', b'b', b'c'];
        let mut result = String::with_capacity(n);
        let mut prev = None;

        for pos in 0..n {
            let block_size = if pos + 1 == n {
                1
            } else {
                1_u64 << (n - pos - 1)
            };

            let mut choices = [0_u8; 3];
            let mut count = 0;

            for &letter in &letters {
                if Some(letter) != prev {
                    choices[count] = letter;
                    count += 1;
                }
            }

            let choice_index = (k / block_size) as usize;
            let chosen = choices[choice_index];
            result.push(chosen as char);
            prev = Some(chosen);
            k %= block_size;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::the_k_th_lexicographical_string_of_all_happy_strings_of_length_n::Solution;

    #[test]
    fn test_get_happy_string_1() {
        let n = 1;
        let k = 3;
        assert_eq!("c".to_string(), Solution::get_happy_string(n, k));
    }

    #[test]
    fn test_get_happy_string_2() {
        let n = 1;
        let k = 4;
        assert_eq!("".to_string(), Solution::get_happy_string(n, k));
    }

    #[test]
    fn test_get_happy_string_3() {
        let n = 3;
        let k = 9;
        assert_eq!("cab".to_string(), Solution::get_happy_string(n, k));
    }
}
