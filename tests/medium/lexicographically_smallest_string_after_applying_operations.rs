// 1625. Lexicographically Smallest String After Applying Operations
// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut smallest = s.clone();

        queue.push_back(s.clone());
        visited.insert(s);

        while let Some(current) = queue.pop_front() {
            // Update smallest if current is smaller
            if current < smallest {
                smallest = current.clone();
            }

            // Operation 1: Add a to all odd indices
            let added = Self::add_operation(&current, a);
            if visited.insert(added.clone()) {
                queue.push_back(added);
            }

            // Operation 2: Rotate right by b positions
            let rotated = Self::rotate_operation(&current, b as usize);
            if visited.insert(rotated.clone()) {
                queue.push_back(rotated);
            }
        }

        smallest
    }

    fn add_operation(s: &str, a: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();

        // Add a to all odd indices (1, 3, 5, ...)
        for i in (1..chars.len()).step_by(2) {
            let digit = chars[i].to_digit(10).unwrap() as i32;
            let new_digit = (digit + a) % 10;
            chars[i] = char::from_digit(new_digit as u32, 10).unwrap();
        }

        chars.into_iter().collect()
    }

    fn rotate_operation(s: &str, b: usize) -> String {
        let len = s.len();
        let b = b % len; // Handle case where b >= len

        // Rotate right by b means take last b chars and put them at front
        let split_point = len - b;
        format!("{}{}", &s[split_point..], &s[..split_point])
    }

    pub fn _find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        println!("s: {}, a: {}, b: {}", &s, &a, &b);

        let mut n = s
            .chars()
            .into_iter()
            .map(|c| c as i32 - 48)
            .collect::<Vec<i32>>();

        let mut map: HashSet<Vec<i32>> = HashSet::new();
        let mut min = n.clone();

        loop {
            loop {
                if map.contains(&n) {
                    break;
                }

                map.insert(n.clone());
                min = Self::_compare(n.clone(), min);

                // next
                for i in 0..n.len() {
                    if i % 2 == 1 {
                        n[i] = (n[i] + a) % 10;
                    }
                }
            }

            let mut d = vec![];
            d.extend_from_slice(&n[n.len() - b as usize..]);
            d.extend_from_slice(&n[0..n.len() - b as usize]);
            n = d;

            if map.contains(&n) {
                break;
            } else {
                map.insert(n.clone());
                min = Self::_compare(n.clone(), min);
            }
        }

        println!("n: {:?}", &n);

        min.iter().map(|&x| (x as u8 + 48) as char).collect()
    }

    fn _compare(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        for i in 0..a.len() {
            if a[i] < b[i] {
                return a;
            } else if a[i] > b[i] {
                return b;
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::lexicographically_smallest_string_after_applying_operations::Solution;

    #[test]
    fn test_find_lex_smallest_string_1() {
        let s = "5525".to_string();
        let a = 9;
        let b = 2;
        assert_eq!(
            "2050".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }

    #[test]
    fn test_find_lex_smallest_string_2() {
        let s = "74".to_string();
        let a = 5;
        let b = 1;
        assert_eq!(
            "24".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }

    #[test]
    fn test_find_lex_smallest_string_3() {
        let s = "0011".to_string();
        let a = 4;
        let b = 2;
        assert_eq!(
            "0011".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }
}
