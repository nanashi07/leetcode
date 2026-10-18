// 1569. Number of Ways to Reorder Array to Get Same BST
// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    /// The root is fixed as the first element, then the elements smaller and larger than it keep
    /// their relative order inside the left and right subtrees. For a subtree of `m` nodes whose
    /// left side holds `l` nodes there are C(m - 1, l) interleavings, multiplied by the counts of
    /// both sides. Subtract 1 at the end to drop the original ordering itself.
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        // Binomial coefficients from Pascal's triangle; n <= 1000 << MOD so none of them is 0.
        let mut comb: Vec<Vec<i64>> = vec![vec![1]];
        for i in 1..=n {
            let prev = &comb[i - 1];
            let mut row = vec![1; i + 1];
            for j in 1..i {
                row[j] = (prev[j - 1] + prev[j]) % MOD;
            }
            comb.push(row);
        }

        // Explicit stack instead of recursion: a sorted input would nest n levels deep.
        let mut ans: i64 = 1;
        let mut stack: Vec<Vec<i32>> = vec![nums];
        while let Some(sub) = stack.pop() {
            if sub.len() < 2 {
                continue;
            }
            let root = sub[0];
            let left: Vec<i32> = sub[1..].iter().copied().filter(|x| *x < root).collect();
            let right: Vec<i32> = sub[1..].iter().copied().filter(|x| *x > root).collect();
            ans = ans * comb[sub.len() - 1][left.len()] % MOD;
            if left.len() > 1 {
                stack.push(left);
            }
            if right.len() > 1 {
                stack.push(right);
            }
        }

        (ans - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_reorder_array_to_get_same_bst::Solution;

    #[test]
    fn test_num_of_ways_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(1, Solution::num_of_ways(nums));
    }

    #[test]
    fn test_num_of_ways_2() {
        let nums = [3, 4, 5, 1, 2].to_vec();
        assert_eq!(5, Solution::num_of_ways(nums));
    }

    #[test]
    fn test_num_of_ways_3() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(0, Solution::num_of_ways(nums));
    }
}
