// 3942. Minimum Operations to Sort a Permutation
// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation/

struct Solution;

impl Solution {
    // Rotations and reversals generate the dihedral group, so `nums` is sortable only when its
    // values walk the cycle uniformly by +1 (a rotation of the sorted array) or by -1 (a rotation
    // of the reversed sorted array). A reversal conjugates a left rotation into a right one
    // (`V R^k V == R^-k`), which is why a forward rotation may cost `+2` extra reversals.
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let at = |i: i64| nums[i.rem_euclid(n) as usize] as i64;

        let step = (at(1) - at(0)).rem_euclid(n);
        if !(1..n).all(|i| (at(i + 1) - at(i)).rem_euclid(n) == step) {
            return -1;
        }

        let mut ans = i64::MAX;
        if step == 1 % n {
            // Rotating left by the index of `0` sorts it; going the other way needs two reversals.
            let j = nums.iter().position(|&v| v == 0).unwrap_or_default() as i64;
            ans = ans.min(j.min(n - j + 2));
        }
        if step == (n - 1) % n {
            // One reversal, plus the cheapest rotation aligning `nums[0]` with its final slot.
            let j = (-at(0) - 1).rem_euclid(n);
            ans = ans.min(j.min(n - j) + 1);
        }

        if ans == i64::MAX {
            -1
        } else {
            ans as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_sort_a_permutation::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [0, 2, 1].to_vec();
        assert_eq!(2, Solution::min_operations(nums.clone()));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [1, 0, 2].to_vec();
        assert_eq!(2, Solution::min_operations(nums.clone()));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = [2, 0, 1, 3].to_vec();
        assert_eq!(-1, Solution::min_operations(nums.clone()));
    }
}
