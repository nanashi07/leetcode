// 1925. Count Square Sum Triples
// https://leetcode.com/problems/count-square-sum-triples/

struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        println!("n: {n}");

        let mut count = 0;

        for a in 1..n {
            for b in 1..n {
                let c = ((a * a + b * b) as f64).sqrt();
                if c.floor() == c && c <= n as f64 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_square_sum_triples::Solution;

    #[test]
    fn test_count_triples_1() {
        let n = 5;
        assert_eq!(2, Solution::count_triples(n));
    }

    #[test]
    fn test_count_triples_2() {
        let n = 10;
        assert_eq!(4, Solution::count_triples(n));
    }
}
