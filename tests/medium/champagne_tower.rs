// 799. Champagne Tower
// https://leetcode.com/problems/champagne-tower/

struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        // Create a 2D array to store the amount of champagne in each glass
        // We only need query_row + 1 rows
        let mut tower: Vec<Vec<f64>> = vec![vec![0.0; 102]; 102];

        // Pour all champagne into the top glass
        tower[0][0] = poured as f64;

        // Simulate the pouring process row by row
        for row in 0..=query_row as usize {
            for col in 0..=row {
                // If this glass has more than 1.0, overflow to the glasses below
                if tower[row][col] > 1.0 {
                    let overflow = tower[row][col] - 1.0;
                    tower[row][col] = 1.0;

                    // Split overflow equally to the two glasses below
                    tower[row + 1][col] += overflow / 2.0;
                    tower[row + 1][col + 1] += overflow / 2.0;
                }
            }
        }

        // Return the amount in the queried glass, capped at 1.0
        tower[query_row as usize][query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::champagne_tower::Solution;

    #[test]
    fn test_champagne_tower_1() {
        let oured = 1;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(
            0.00000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }

    #[test]
    fn test_champagne_tower_2() {
        let oured = 2;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(
            0.50000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }

    #[test]
    fn test_champagne_tower_3() {
        let oured = 100000009;
        let query_row = 33;
        let query_glass = 17;
        assert_eq!(
            1.00000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }
}
