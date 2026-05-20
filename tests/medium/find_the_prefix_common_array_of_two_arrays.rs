// 2657. Find the Prefix Common Array of Two Arrays
// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/

struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut seen_a = vec![false; n + 1];
        let mut seen_b = vec![false; n + 1];
        let mut common = 0;
        let mut result = Vec::with_capacity(n);

        for i in 0..n {
            let x = a[i] as usize;
            let y = b[i] as usize;

            if seen_b[x] {
                common += 1;
            }
            seen_a[x] = true;

            if seen_a[y] {
                common += 1;
            }
            seen_b[y] = true;

            result.push(common);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_prefix_common_array_of_two_arrays::Solution;

    #[test]
    fn test_find_the_prefix_common_array_1() {
        let a = [1, 3, 2, 4].to_vec();
        let b = [3, 1, 2, 4].to_vec();
        assert_eq!(
            [0, 2, 3, 4].to_vec(),
            Solution::find_the_prefix_common_array(a, b)
        );
    }

    #[test]
    fn test_find_the_prefix_common_array_2() {
        let a = [2, 3, 1].to_vec();
        let b = [3, 1, 2].to_vec();
        assert_eq!(
            [0, 1, 3].to_vec(),
            Solution::find_the_prefix_common_array(a, b)
        );
    }
}
