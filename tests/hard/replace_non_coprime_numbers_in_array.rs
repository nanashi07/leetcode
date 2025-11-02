// 2197. Replace Non-Coprime Numbers in Array
// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/

struct Solution;
impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for num in nums {
            let mut current = num;

            // Keep trying to merge current with the last element in result
            while let Some(&last) = result.last() {
                let g = Self::gcd(current, last);
                if g == 1 {
                    // They are coprime, can't merge
                    break;
                }
                // They are not coprime, merge them
                result.pop();
                current = Self::lcm(current, last);
            }

            result.push(current);
        }

        result
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        (a as i64 * b as i64 / Self::gcd(a, b) as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::replace_non_coprime_numbers_in_array::Solution;

    #[test]
    fn test_replace_non_coprimes_1() {
        let nums = [6, 4, 3, 2, 7, 6, 2].to_vec();
        assert_eq!([12, 7, 6].to_vec(), Solution::replace_non_coprimes(nums));
    }

    #[test]
    fn test_replace_non_coprimes_2() {
        let nums = [2, 2, 1, 1, 3, 3, 3].to_vec();
        assert_eq!([2, 1, 1, 3].to_vec(), Solution::replace_non_coprimes(nums));
    }
}
