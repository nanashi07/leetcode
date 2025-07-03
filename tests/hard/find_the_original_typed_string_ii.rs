// # 3333. Find the Original Typed String II
// https://leetcode.com/problems/find-the-original-typed-string-ii/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        println!("word: {}, k: {}", &word, &k);
        const MOD: i32 = 10_i32.pow(9) + 7;

        let mut counter: Vec<i32> = vec![];
        let mut last_c: Option<char> = None;
        for c in word.chars() {
            if last_c == Some(c) {
                let l = counter.len();
                counter[l - 1] += 1;
            } else {
                counter.push(1);
                last_c = Some(c);
            }
        }
        println!("counter: {:?}", &counter);

        let mut hash: HashSet<String> = HashSet::new();
        let min = counter.len() as i32;
        let mut num = 0;
        for i in 0..k - min {
            num += Self::count_x(
                &mut hash,
                &counter.iter().map(|x| *x - 1).collect::<Vec<i32>>(),
                i,
            );
        }

        println!("num: {:?}", &num);
        println!("ccc: {:?}", counter.iter().fold(1, |acc, x| acc * x));
        println!("hash: {:?}", &hash);
        // num
        counter.iter().fold(1, |acc, x| acc * x % MOD) - (hash.len() as i32)
    }

    fn count_x(hash: &mut HashSet<String>, ct: &[i32], x: i32) -> i32 {
        println!("ctt: {:?}, x: {}", &ct, x);
        let mut sum = 0;
        if x > 0 {
            for i in 0..ct.len() {
                if ct[i] > 0 {
                    let mut ctt = ct.to_vec();
                    ctt[i] -= 1;
                    sum += Self::count_x(hash, &ctt, x - 1);
                }
            }
        } else {
            println!("xx: {:?}", &ct);
            hash.insert(format!("{:?}", &ct));
            return 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_original_typed_string_ii::Solution;

    #[test]
    fn test_possible_string_count_1() {
        let word = "aabbccdd".to_owned();
        let k = 7;
        assert_eq!(5, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_2() {
        let word = "aabbccdd".to_owned();
        let k = 8;
        assert_eq!(1, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_3() {
        let word = "aaabbb".to_owned();
        // aaabbb, aabbb, abbb,  | [0,0], [1,0], [2,0]
        // aaabb,  aabb,  abb,   | [0,1], [1,1], [2,1]
        // aaab,   aab,  [ab]    | [0,2], [1,2]m [2,2]
        let k = 3;
        assert_eq!(8, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_4() {
        let word = "bbbblllliizzzznnnnna".to_owned();
        // aaabbb, aabbb, abbb,  | [0,0], [1,0], [2,0]
        // aaabb,  aabb,  abb,   | [0,1], [1,1], [2,1]
        // aaab,   aab,  [ab]    | [0,2], [1,2]m [2,2]
        let k = 20;
        assert_eq!(8, Solution::possible_string_count(word, k));
    }
}
