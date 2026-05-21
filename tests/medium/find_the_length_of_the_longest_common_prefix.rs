// 3043. Find the Length of the Longest Common Prefix
// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/

struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        fn digits_count(mut n: i32) -> i32 {
            if n == 0 {
                return 1;
            }

            let mut count = 0;
            while n > 0 {
                count += 1;
                n /= 10;
            }
            count
        }

        let mut prefixes = HashSet::new();

        for mut n in arr1 {
            if n == 0 {
                prefixes.insert(0);
                continue;
            }

            while n > 0 {
                prefixes.insert(n);
                n /= 10;
            }
        }

        let mut best = 0;

        for n in arr2 {
            if n == 0 {
                if prefixes.contains(&0) {
                    best = best.max(1);
                }
                continue;
            }

            let mut cur = n;
            let mut len = digits_count(n);
            while cur > 0 {
                if prefixes.contains(&cur) {
                    best = best.max(len);
                    break;
                }
                cur /= 10;
                len -= 1;
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_length_of_the_longest_common_prefix::Solution;

    #[test]
    fn test_longest_common_prefix_1() {
        let arr1 = [1, 10, 100].to_vec();
        let arr2 = [1000].to_vec();
        assert_eq!(3, Solution::longest_common_prefix(arr1, arr2));
    }

    #[test]
    fn test_longest_common_prefix_2() {
        let arr1 = [1, 2, 3].to_vec();
        let arr2 = [4, 4, 4].to_vec();
        assert_eq!(0, Solution::longest_common_prefix(arr1, arr2));
    }
}
