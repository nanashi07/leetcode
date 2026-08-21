// 3116. Kth Smallest Amount With Single Denomination Combination
// https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination/

struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let coins: Vec<i64> = coins.into_iter().map(i64::from).collect();
        let n = coins.len();

        // Precompute LCMs for all non-empty subsets via bitmask
        let mut lcms = Vec::with_capacity((1 << n) - 1);
        for mask in 1..(1i32 << n) {
            let mut l: i64 = 1;
            let mut bits = 0u32;
            let mut overflow = false;
            for (i, &c) in coins.iter().enumerate() {
                if mask >> i & 1 == 1 {
                    bits += 1;
                    l = l / Self::gcd(l, c) * c;
                    if l > 1_000_000_000_000_000 {
                        overflow = true;
                        break;
                    }
                }
            }
            lcms.push((l, bits, overflow));
        }

        // Binary search: find smallest x with count(x) >= k
        let mut lo: i64 = 1;
        let mut hi: i64 = *coins.iter().min().unwrap() * k;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = Self::count(&lcms, mid);
            if cnt >= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    /// Count how many numbers in [1, x] are multiples of at least one coin (inclusion-exclusion).
    fn count(lcms: &[(i64, u32, bool)], x: i64) -> i64 {
        let mut total: i64 = 0;
        for &(l, bits, overflow) in lcms {
            if overflow {
                continue;
            }
            if bits % 2 == 1 {
                total += x / l;
            } else {
                total -= x / l;
            }
        }
        total
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::kth_smallest_amount_with_single_denomination_combination::Solution;

    #[test]
    fn test_find_kth_smallest_1() {
        let coins = [3, 6, 9].to_vec();
        let k = 3;
        assert_eq!(9, Solution::find_kth_smallest(coins, k));
    }

    #[test]
    fn test_find_kth_smallest_2() {
        let coins = [5, 2].to_vec();
        let k = 7;
        assert_eq!(12, Solution::find_kth_smallest(coins, k));
    }
}
