// 868. Binary Gap
// https://leetcode.com/problems/binary-gap/

struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        println!("n: {n} ({n:b})");

        let mut n = n;

        while n > 0 && n % 2 == 0 {
            n = n >> 1;
        }

        let mut p = -1;
        let mut last = 0;
        let mut max = -1;

        while n > 0 {
            p += 1;
            let len = if n % 2 == 1 {
                let l = p - last;
                last = p;
                l
            } else {
                p - last
            };
            max = max.max(len);
            n = n >> 1;
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_gap::Solution;

    #[test]
    fn test_binary_gap_1() {
        let n = 22;
        assert_eq!(2, Solution::binary_gap(n));
    }

    #[test]
    fn test_binary_gap_2() {
        let n = 8;
        assert_eq!(0, Solution::binary_gap(n));
    }

    #[test]
    fn test_binary_gap_3() {
        let n = 5;
        assert_eq!(2, Solution::binary_gap(n));
    }

    #[test]
    fn test_binary_gap_4() {
        let n = 6;
        assert_eq!(1, Solution::binary_gap(n));
    }
}
