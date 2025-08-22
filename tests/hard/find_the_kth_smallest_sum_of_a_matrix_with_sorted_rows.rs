// # 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows
// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;

        // Start with the first row - these are our initial possible sums
        let mut current_sums = mat[0].clone();
        current_sums.sort_unstable();

        // Keep only the k smallest sums from first row
        if current_sums.len() > k {
            current_sums.truncate(k);
        }

        // Process each subsequent row
        for row in 1..mat.len() {
            current_sums = Self::merge_with_row(&current_sums, &mat[row], k);
        }

        current_sums[k - 1]
    }

    // Merge current k smallest sums with a new row, return k smallest results
    fn merge_with_row(current_sums: &[i32], row: &[i32], k: usize) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        // Start with combinations of each current sum with the first element of new row
        for (sum_idx, &sum) in current_sums.iter().enumerate() {
            heap.push(Reverse((sum + row[0], sum_idx, 0)));
        }

        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // Mark initial states as visited
        for sum_idx in 0..current_sums.len() {
            visited.insert((sum_idx, 0));
        }

        while result.len() < k && !heap.is_empty() {
            if let Some(Reverse((sum, sum_idx, row_idx))) = heap.pop() {
                result.push(sum);

                // Add the next element from the same row (same sum_idx, row_idx + 1)
                if row_idx + 1 < row.len() && !visited.contains(&(sum_idx, row_idx + 1)) {
                    visited.insert((sum_idx, row_idx + 1));
                    heap.push(Reverse((
                        current_sums[sum_idx] + row[row_idx + 1],
                        sum_idx,
                        row_idx + 1,
                    )));
                }

                // Add the same row position with next sum (sum_idx + 1, row_idx)
                if sum_idx + 1 < current_sums.len() && !visited.contains(&(sum_idx + 1, row_idx)) {
                    visited.insert((sum_idx + 1, row_idx));
                    heap.push(Reverse((
                        current_sums[sum_idx + 1] + row[row_idx],
                        sum_idx + 1,
                        row_idx,
                    )));
                }
            }
        }

        result
    }

    pub fn _kth_smallest4(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let outer_len = mat.len();
        let inner_len = mat[0].len();
        println!("mat: {:?}", &mat);
        // let mut diff: Vec<Vec<i32>> = vec![vec![0; inner_len]; outer_len];

        // for o in 0..outer_len {
        //     for i in 1..inner_len {
        //         diff[o][i] = mat[o][i] - mat[o][0];
        //     }
        // }
        // println!("diff: {:?}", &diff);

        let mut indexes = vec![0; outer_len];
        let mut sums: Vec<i32> = Vec::new();
        sums.push(mat.iter().fold(0, |acc, x| acc + x[0]));

        while (sums.len() as i32) <= 2 * k {
            // println!("indexes: {:?}", &indexes);

            // iterator all possible next sums

            for changing_row in 0..outer_len {
                for i in 0..outer_len {
                    let mut base = sums[0];
                    let mut _temp: Vec<i32> = Vec::new();

                    // iterator changing row all remained elements
                    if i == changing_row {
                        // iterator to the end
                        for _j in indexes[i]..inner_len {}
                    } else {
                        base += mat[i][indexes[i]];
                    }

                    println!("temp: {:?}, sum = {}", &_temp, base);
                    sums.push(base);
                }
            }

            // move index to the next min value
            let mut p = 0;
            let mut min = i32::MAX;
            for i in 0..outer_len {
                let next = indexes[i] + 1;
                if next < inner_len && min > mat[i][next] {
                    p = i;
                    min = mat[i][next];
                }
            }

            indexes[p] += 1;
            let mut moved: Vec<i32> = Vec::new();
            for i in 0..outer_len {
                moved.push(mat[i][indexes[i]])
            }
            println!("Move to {:?}, index: {:?}", &moved, &indexes);
            // println!("update indexes: {:?}", &indexes);
        }

        sums.sort();
        println!("sums: {:?}", &sums);
        *sums.iter().take(k as usize).last().unwrap()
    }

    // too slow
    pub fn _kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        // let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut heap: Vec<i32> = Vec::new();
        let outer_len = mat.len();
        let inner_len = mat[0].len();
        println!("mat: {:?}", &mat);

        let mut indexes = vec![0; outer_len];

        while indexes[0] < inner_len {
            let mut sum = 0;
            let mut sub: Vec<i32> = Vec::new();

            // println!("index: {:?}", &indexes);

            for i in (0..outer_len).rev() {
                let value = mat[i][indexes[i]];
                sum += value;
                sub.insert(0, value);
            }

            // index + 1, individual rounding
            indexes[outer_len - 1] += 1;
            for i in (1..outer_len).rev() {
                if indexes[i] >= inner_len {
                    indexes[i - 1] += 1;
                    indexes[i] = 0;
                }
            }

            heap.push(sum);
            println!("{:?}, sum = {}", sub, sum);
        }

        heap.sort();
        println!("heap: {:?}", &heap);
        *heap.iter().take(k as usize).last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows::Solution;

    #[test]
    fn test_kth_smallest_1() {
        let mat = [[1, 3, 11], [2, 4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let k = 5;
        assert_eq!(7, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_2() {
        let mat = [[1, 3, 11], [2, 4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let k = 9;
        assert_eq!(17, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_3() {
        let mat = [[1, 10, 10], [1, 4, 5], [2, 3, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let k = 7;
        assert_eq!(9, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_4() {
        let mat = [
            [21, 33, 38, 41, 42, 44, 48, 54, 62],
            [8, 51, 56, 60, 80, 81, 83, 88, 92],
            [2, 40, 54, 56, 68, 92, 93, 94, 98],
            [28, 35, 41, 52, 55, 59, 62, 70, 99],
            [2, 16, 25, 30, 32, 38, 54, 73, 92],
            [1, 11, 14, 35, 35, 55, 59, 66, 66],
            [5, 9, 13, 29, 47, 49, 60, 73, 74],
            [19, 29, 32, 36, 59, 64, 66, 98, 100],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect::<Vec<_>>();
        let k = 29;
        assert_eq!(106, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_5() {
        let mat = [[1, 10, 10], [1, 4, 5], [2, 3, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let k = 14;
        assert_eq!(16, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_6() {
        let mat = [[2, 5, 6], [1, 5, 8], [2, 3, 7]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let k = 4;
        assert_eq!(9, Solution::kth_smallest(mat, k));
    }
}
