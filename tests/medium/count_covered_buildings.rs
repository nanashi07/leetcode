// 3531. Count Covered Buildings
// https://leetcode.com/problems/count-covered-buildings/

struct Solution;

impl Solution {
    pub fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
        for coord in buildings {
            if coord.len() < 2 {
                continue;
            }
            let row = coord[0];
            let col = coord[1];
            *counts.entry((row, col)).or_insert(0) += 1;
        }
        if counts.is_empty() {
            return 0;
        }

        let mut row_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut col_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (&(row, col), _) in counts.iter() {
            row_map.entry(row).or_default().push(col);
            col_map.entry(col).or_default().push(row);
        }
        for cols in row_map.values_mut() {
            cols.sort_unstable();
            cols.dedup();
        }
        for rows in col_map.values_mut() {
            rows.sort_unstable();
            rows.dedup();
        }

        fn has_neighbors(sorted: &[i32], value: i32) -> bool {
            if let Ok(index) = sorted.binary_search(&value) {
                index > 0 && index + 1 < sorted.len()
            } else {
                false
            }
        }

        let mut covered = 0;
        for ((row, col), freq) in counts.into_iter() {
            let row_neighbors = row_map
                .get(&row)
                .map(|cols| has_neighbors(cols, col))
                .unwrap_or(false);
            if !row_neighbors {
                continue;
            }
            let col_neighbors = col_map
                .get(&col)
                .map(|rows| has_neighbors(rows, row))
                .unwrap_or(false);
            if row_neighbors && col_neighbors {
                covered += freq;
            }
        }

        covered
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_covered_buildings::Solution;

    #[test]
    fn test_count_covered_buildings_1() {
        let n = 3;
        let buildings = [[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_covered_buildings(n, buildings));
    }

    #[test]
    fn test_count_covered_buildings_2() {
        let n = 3;
        let buildings = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::count_covered_buildings(n, buildings));
    }

    #[test]
    fn test_count_covered_buildings_3() {
        let n = 5;
        let buildings = [[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_covered_buildings(n, buildings));
    }
}
