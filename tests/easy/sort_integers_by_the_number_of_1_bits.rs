// 1356. Sort Integers by The Number of 1 Bits
// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/

struct Solution;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        println!("arr: {arr:?}");

        let mut list = arr
            .iter()
            .map(|n| (n, format!("{n:b}").replace("0", "").len()))
            .collect::<Vec<_>>();
        list.sort_by(|(n1, c1), (n2, c2)| c1.cmp(c2).then(n1.cmp(n2)));

        list.iter().map(|(n, _)| **n).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::sort_integers_by_the_number_of_1_bits::Solution;

    #[test]
    fn test_sort_by_bits_1() {
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec();
        let output = [0, 1, 2, 4, 8, 3, 5, 6, 7].to_vec();
        assert_eq!(output, Solution::sort_by_bits(arr));
    }

    #[test]
    fn test_sort_by_bits_2() {
        let arr = [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1].to_vec();
        let output = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024].to_vec();
        assert_eq!(output, Solution::sort_by_bits(arr));
    }
}
