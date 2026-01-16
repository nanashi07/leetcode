// 2975. Maximum Square Area by Removing Fences From a Field
// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/

struct Solution;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        const MOD: i64 = 1_000_000_007;

        // Add boundaries to fences
        let mut h_positions = h_fences.clone();
        h_positions.push(1);
        h_positions.push(m);
        h_positions.sort_unstable();

        let mut v_positions = v_fences.clone();
        v_positions.push(1);
        v_positions.push(n);
        v_positions.sort_unstable();

        // Find all possible horizontal distances (heights)
        let mut h_distances = HashSet::new();
        for i in 0..h_positions.len() {
            for j in i + 1..h_positions.len() {
                h_distances.insert(h_positions[j] - h_positions[i]);
            }
        }

        // Find all possible vertical distances (widths)
        let mut v_distances = HashSet::new();
        for i in 0..v_positions.len() {
            for j in i + 1..v_positions.len() {
                v_distances.insert(v_positions[j] - v_positions[i]);
            }
        }

        // Find the maximum common distance (side length for square)
        let mut max_side = -1;
        for &dist in &h_distances {
            if v_distances.contains(&dist) {
                max_side = max_side.max(dist);
            }
        }

        if max_side == -1 {
            -1
        } else {
            let area = (max_side as i64 * max_side as i64) % MOD;
            area as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_square_area_by_removing_fences_from_a_field::Solution;

    #[test]
    fn test_maximize_square_area_1() {
        let m = 4;
        let n = 3;
        let h_fences = [2, 3].to_vec();
        let v_fences = [2].to_vec();
        assert_eq!(4, Solution::maximize_square_area(m, n, h_fences, v_fences));
    }

    #[test]
    fn test_maximize_square_area_2() {
        let m = 6;
        let n = 7;
        let h_fences = [2].to_vec();
        let v_fences = [4].to_vec();
        assert_eq!(-1, Solution::maximize_square_area(m, n, h_fences, v_fences));
    }
}
