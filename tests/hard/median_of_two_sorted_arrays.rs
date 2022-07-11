// # 4. Median of Two Sorted Arrays
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();

        let mid = (l1 + l2) as f32 / 2.0;
        let mut i1 = 0;
        let mut i2 = 0;
        let mut n = f64::NAN;

        for i in 0..(l1 + l2) {
            let n1 = if i1 < l1 { nums1[i1] as f64 } else { f64::NAN };
            let n2 = if i2 < l2 { nums2[i2] as f64 } else { f64::NAN };

            if n1.is_nan() || n1 > n2 {
                n = n2;
                i2 = i2 + 1;
            } else if n2.is_nan() || n1 <= n2 {
                n = n1;
                i1 = i1 + 1;
            }

            // x.5, odd count
            if mid - 1.0 < i as f32 {
                return n as f64;
            } else if mid - 1.0 == i as f32 {
                // get next
                let n1 = if i1 < l1 { nums1[i1] as f64 } else { f64::NAN };
                let n2 = if i2 < l2 { nums2[i2] as f64 } else { f64::NAN };

                return (n + f64::min(n1, n2)) as f64 / 2.0;
            }
        }

        0.0
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.0, result);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.5, result);
}
