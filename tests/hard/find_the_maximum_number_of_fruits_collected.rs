// # 3363. Find the Maximum Number of Fruits Collected
// https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected/

struct Solution;

fn dp(fruits: &Vec<Vec<i32>>, n: usize) -> i32 {
    let mut prev = vec![i32::MIN; n];
    let mut curr = vec![i32::MIN; n];

    prev[n - 1] = fruits[0][n - 1];
    for i in 1..n - 1 {
        for j in (n - 1 - i).max(i + 1)..n {
            let mut best = prev[j];
            if j > 0 {
                best = best.max(prev[j - 1]);
            }
            if j + 1 < n {
                best = best.max(prev[j + 1]);
            }
            curr[j] = best + fruits[i][j];
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n - 1]
}

impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut ans = (0..n).map(|i| fruits[i][i]).sum::<i32>();

        ans += dp(&fruits, n);
        for i in 0..n {
            for j in 0..i {
                let tmp = fruits[i][j];
                fruits[i][j] = fruits[j][i];
                fruits[j][i] = tmp;
            }
        }
        ans += dp(&fruits, n);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_maximum_number_of_fruits_collected::Solution;

    #[test]
    fn test_max_collected_fruits_1() {
        let fruits = [
            [1, 2, 3, 4],
            [5, 6, 8, 7],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|&l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        assert_eq!(100, Solution::max_collected_fruits(fruits));
    }

    #[test]
    fn test_max_collected_fruits_2() {
        let fruits = [[1, 1], [1, 1]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::max_collected_fruits(fruits));
    }
}
