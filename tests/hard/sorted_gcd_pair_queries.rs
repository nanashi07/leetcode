// 3312. Sorted GCD Pair Queries
// https://leetcode.com/problems/sorted-gcd-pair-queries/

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max_val = *nums.iter().max().unwrap_or(&0) as usize;
        if max_val == 0 {
            return vec![];
        }

        let mut count = vec![0_i64; max_val + 1];
        for &x in nums.iter() {
            count[x as usize] += 1;
        }

        let mut pairs_exact = vec![0_i64; max_val + 1];
        for i in (1..=max_val).rev() {
            let mut multiples = 0;
            for j in (i..=max_val).step_by(i) {
                multiples += count[j];
            }
            let mut exact = multiples * (multiples - 1) / 2;
            for j in ((i * 2)..=max_val).step_by(i) {
                exact -= pairs_exact[j];
            }
            pairs_exact[i] = exact;
        }

        let mut prefix = vec![0_i64; max_val + 1];
        for i in 1..=max_val {
            prefix[i] = prefix[i - 1] + pairs_exact[i];
        }

        queries
            .into_iter()
            .map(|q| prefix.partition_point(|&x| x <= q) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::sorted_gcd_pair_queries::Solution;

    #[test]
    fn test_gcd_values_1() {
        let nums = [2, 3, 4].to_vec();
        let queries = [0, 2, 2].to_vec();
        assert_eq!([1, 2, 2].to_vec(), Solution::gcd_values(nums, queries));
    }

    #[test]
    fn test_gcd_values_2() {
        let nums = [4, 4, 2, 1].to_vec();
        let queries = [5, 3, 1, 0].to_vec();
        assert_eq!([4, 2, 1, 1].to_vec(), Solution::gcd_values(nums, queries));
    }

    #[test]
    fn test_gcd_values_3() {
        let nums = [2, 2].to_vec();
        let queries = [0, 0].to_vec();
        assert_eq!([2, 2].to_vec(), Solution::gcd_values(nums, queries));
    }
}
