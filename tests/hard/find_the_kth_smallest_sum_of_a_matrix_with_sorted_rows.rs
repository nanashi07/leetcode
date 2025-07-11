// # 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows
// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/

struct Solution;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let outer_len = mat.len();
        let inner_len = mat[0].len();
        println!("mat: {:?}", &mat);

        let mut sums: Vec<i32> = Vec::new();
        let mut indexes = vec![0; outer_len];

        let take_sum = |slices: &Vec<usize>| {
            let mut sum = 0;
            let mut sub: Vec<i32> = Vec::new();
            for i in 0..outer_len {
                sub.push(mat[i][slices[i]]);
                sum += mat[i][slices[i]];
            }
            println!("sub: {:?}, sum = {}", &sub, sum);
            sum
        };

        sums.sort();
        let mut max_sum = 0;

        let init_sum = take_sum(&indexes);
        while (sums.len() < k as usize) || max_sum >= init_sum {
            // println!("indexes: {:?}", &indexes);
            sums.push(init_sum);

            for moving_down in 0..outer_len {
                let index = indexes[moving_down];

                for moving_right in index + 1..inner_len {
                    let mut another_indexes = indexes.clone();
                    another_indexes[moving_down] = moving_right;

                    let another_sum = take_sum(&another_indexes);
                    if (sums.len() < k as usize) || another_sum <= max_sum {
                        sums.push(another_sum);
                    } else {
                        break;
                    }
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

            sums.sort();
            max_sum = *sums.iter().take(k as usize).max().unwrap();
        }
        println!("sums: {:?}", &sums);
        *sums.iter().take(k as usize).last().unwrap()
    }

    pub fn kth_smallest4(mat: Vec<Vec<i32>>, k: i32) -> i32 {
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
                    let mut temp: Vec<i32> = Vec::new();

                    // iterator changing row all remained elements
                    if i == changing_row {
                        // iterator to the end
                        for j in indexes[i]..inner_len {}
                    } else {
                        base += mat[i][indexes[i]];
                    }

                    println!("temp: {:?}, sum = {}", &temp, base);
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
        assert_eq!(141, Solution::kth_smallest(mat, k));
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
