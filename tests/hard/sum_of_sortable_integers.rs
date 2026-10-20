// 3886. Sum of Sortable Integers
// https://leetcode.com/problems/sum-of-sortable-integers/

struct Solution;

impl Solution {
    /// True when every block of length `k` can be cyclically rotated into the
    /// matching segment of `sorted`, which is the only possible final array.
    fn blocks_rotate_into(nums: &[i32], sorted: &[i32], diff: &[usize], k: usize) -> bool {
        let mut lps: Vec<usize> = Vec::new();

        for (index, (block, target)) in nums.chunks(k).zip(sorted.chunks(k)).enumerate() {
            // A block that already sits on its target segment needs no rotation.
            let start = index * k;
            if diff[start + k] == diff[start] {
                continue;
            }

            // Only index 0 is not rewritten by the loop below.
            lps.resize(k, 0);
            lps[0] = 0;

            // KMP table of the target segment.
            for i in 1..k {
                let mut j = lps[i - 1];
                while j > 0 && target[i] != target[j] {
                    j = lps[j - 1];
                }
                if target[i] == target[j] {
                    j += 1;
                }
                lps[i] = j;
            }

            // `target` fits in a rotation of `block` iff it occurs inside the
            // block concatenated with itself; 2 * k - 1 characters suffice.
            let mut matched = false;
            let mut j = 0usize;
            for t in 0..2 * k - 1 {
                let value = block[t % k];
                while j > 0 && value != target[j] {
                    j = lps[j - 1];
                }
                if value == target[j] {
                    j += 1;
                }
                if j == k {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return false;
            }
        }

        true
    }

    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut sorted = nums.clone();
        sorted.sort_unstable();

        // Only divisors of n can partition the array into equal blocks.
        let mut divisors: Vec<usize> = Vec::new();
        let mut d = 1;
        while d * d <= n {
            if n.is_multiple_of(d) {
                divisors.push(d);
                if d * d != n {
                    divisors.push(n / d);
                }
            }
            d += 1;
        }

        // An already sorted array makes every divisor sortable.
        if nums == sorted {
            return divisors.iter().sum::<usize>() as i32;
        }

        // diff[i] counts the positions before i that differ from the sorted array.
        let mut diff: Vec<usize> = Vec::with_capacity(n + 1);
        diff.push(0);
        let mut mismatches = 0usize;
        for (value, target) in nums.iter().zip(&sorted) {
            mismatches += usize::from(value != target);
            diff.push(mismatches);
        }

        let mut answer = 0i64;
        for k in divisors {
            if Self::blocks_rotate_into(&nums, &sorted, &diff, k) {
                answer += k as i64;
            }
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::sum_of_sortable_integers::Solution;

    #[test]
    fn test_sortable_integers_1() {
        let nums = [3, 1, 2].to_vec();
        assert_eq!(3, Solution::sortable_integers(nums));
    }

    #[test]
    fn test_sortable_integers_2() {
        let nums = [7, 6, 5].to_vec();
        assert_eq!(0, Solution::sortable_integers(nums));
    }

    #[test]
    fn test_sortable_integers_3() {
        let nums = [5, 8].to_vec();
        assert_eq!(3, Solution::sortable_integers(nums));
    }
}
