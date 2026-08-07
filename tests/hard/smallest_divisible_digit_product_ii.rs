// 3348. Smallest Divisible Digit Product II
// https://leetcode.com/problems/smallest-divisible-digit-product-ii/

struct Solution;

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        let mut temp = t;
        let mut req2 = 0;
        while temp % 2 == 0 {
            req2 += 1;
            temp /= 2;
        }
        let mut req3 = 0;
        while temp % 3 == 0 {
            req3 += 1;
            temp /= 3;
        }
        let mut req5 = 0;
        while temp % 5 == 0 {
            req5 += 1;
            temp /= 5;
        }
        let mut req7 = 0;
        while temp % 7 == 0 {
            req7 += 1;
            temp /= 7;
        }
        if temp > 1 {
            return "-1".to_string();
        }

        let mut dp = [[u32::MAX; 32]; 50];
        dp[0][0] = 0;
        for i in 0..50 {
            for j in 0..32 {
                if dp[i][j] == u32::MAX {
                    continue;
                }
                for &(c2, c3) in &[(1, 0), (0, 1), (2, 0), (1, 1), (3, 0), (0, 2)] {
                    let ni = (i + c2).min(49);
                    let nj = (j + c3).min(31);
                    dp[ni][nj] = dp[ni][nj].min(dp[i][j] + 1);
                }
            }
        }
        for i in (0..50).rev() {
            for j in (0..32).rev() {
                if i < 49 {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j]);
                }
                if j < 31 {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1]);
                }
            }
        }

        fn get_factors(d: u8) -> (usize, usize, usize, usize) {
            match d {
                2 => (1, 0, 0, 0),
                3 => (0, 1, 0, 0),
                4 => (2, 0, 0, 0),
                5 => (0, 0, 1, 0),
                6 => (1, 1, 0, 0),
                7 => (0, 0, 0, 1),
                8 => (3, 0, 0, 0),
                9 => (0, 2, 0, 0),
                _ => (0, 0, 0, 0),
            }
        }

        let build_greedy = |mut r2: usize,
                            mut r3: usize,
                            mut r5: usize,
                            mut r7: usize,
                            rem_len: usize|
         -> String {
            let mut res = String::with_capacity(rem_len);
            for _ in 0..rem_len {
                for d in 1..=9 {
                    let (f2, f3, f5, f7) = get_factors(d);
                    let nr2 = r2.saturating_sub(f2);
                    let nr3 = r3.saturating_sub(f3);
                    let nr5 = r5.saturating_sub(f5);
                    let nr7 = r7.saturating_sub(f7);
                    if dp[nr2][nr3] as usize + nr5 + nr7 <= rem_len - 1 - res.len() {
                        res.push((d + b'0') as char);
                        r2 = nr2;
                        r3 = nr3;
                        r5 = nr5;
                        r7 = nr7;
                        break;
                    }
                }
            }
            res
        };

        let bytes = num.as_bytes();
        let n = bytes.len();
        let mut k = 0;
        while k < n && bytes[k] != b'0' {
            k += 1;
        }

        let mut has2 = 0;
        let mut has3 = 0;
        let mut has5 = 0;
        let mut has7 = 0;
        for j in 0..k {
            let (f2, f3, f5, f7) = get_factors(bytes[j] - b'0');
            has2 += f2;
            has3 += f3;
            has5 += f5;
            has7 += f7;
        }

        for i in (0..=k).rev() {
            if i == n {
                if req2 <= has2 && req3 <= has3 && req5 <= has5 && req7 <= has7 {
                    return num;
                }
            } else {
                let rem2 = req2.saturating_sub(has2);
                let rem3 = req3.saturating_sub(has3);
                let rem5 = req5.saturating_sub(has5);
                let rem7 = req7.saturating_sub(has7);

                let start_d = 1.max(bytes[i] - b'0' + 1);
                for d in start_d..=9 {
                    let (f2, f3, f5, f7) = get_factors(d);
                    let nr2 = rem2.saturating_sub(f2);
                    let nr3 = rem3.saturating_sub(f3);
                    let nr5 = rem5.saturating_sub(f5);
                    let nr7 = rem7.saturating_sub(f7);

                    let rem_len = n - 1 - i;
                    if dp[nr2][nr3] as usize + nr5 + nr7 <= rem_len {
                        let mut res = String::with_capacity(n);
                        for j in 0..i {
                            res.push(bytes[j] as char);
                        }
                        res.push((d + b'0') as char);
                        res.push_str(&build_greedy(nr2, nr3, nr5, nr7, rem_len));
                        return res;
                    }
                }
            }

            if i > 0 {
                let (f2, f3, f5, f7) = get_factors(bytes[i - 1] - b'0');
                has2 -= f2;
                has3 -= f3;
                has5 -= f5;
                has7 -= f7;
            }
        }

        let new_len = std::cmp::max(n + 1, dp[req2][req3] as usize + req5 + req7);
        build_greedy(req2, req3, req5, req7, new_len)
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_divisible_digit_product_ii::Solution;

    #[test]
    fn test_smallest_number_1() {
        let num = "1234".to_string();
        let t = 256;
        assert_eq!("1488".to_string(), Solution::smallest_number(num, t));
    }

    #[test]
    fn test_smallest_number_2() {
        let num = "12355".to_string();
        let t = 50;
        assert_eq!("12355".to_string(), Solution::smallest_number(num, t));
    }

    #[test]
    fn test_smallest_number_3() {
        let num = "11111".to_string();
        let t = 26;
        assert_eq!("-1".to_string(), Solution::smallest_number(num, t));
    }
}
