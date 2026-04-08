// 3653. XOR After Range Multiplication Queries I
// https://leetcode.com/problems/xor-after-range-multiplication-queries-i/

struct Solution;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        for q in &queries {
            let (l, r, k, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as i64);
            let mut idx = l;
            while idx <= r {
                nums[idx] = ((nums[idx] as i64 * v) % MOD) as i32;
                idx += k;
            }
        }
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::xor_after_range_multiplication_queries_i::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_xor_after_queries_1() {
        let nums = [1, 1, 1].to_vec();
        let queries = to_vec2d([[0, 2, 1, 4]]);
        assert_eq!(4, Solution::xor_after_queries(nums, queries));
    }

    #[test]
    fn test_xor_after_queries_2() {
        let nums = [2, 3, 1, 5, 4].to_vec();
        let queries = to_vec2d([[1, 4, 2, 3], [0, 2, 1, 2]]);
        assert_eq!(31, Solution::xor_after_queries(nums, queries));
    }
}
