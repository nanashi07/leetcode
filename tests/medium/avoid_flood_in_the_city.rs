// # 1488. Avoid Flood in The City
// https://leetcode.com/problems/avoid-flood-in-the-city/

struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::avoid_flood_in_the_city::Solution;

    #[test]
    fn test_avoid_flood_1() {
        let rains = [1, 2, 3, 4].to_vec();
        assert_eq!([-1, -1, -1, -1].to_vec(), Solution::avoid_flood(rains));
    }

    #[test]
    fn test_avoid_flood_2() {
        let rains = [1, 2, 0, 0, 2, 1].to_vec();
        assert_eq!(
            [-1, -1, 2, 1, -1, -1].to_vec(),
            Solution::avoid_flood(rains)
        );
    }

    #[test]
    fn test_avoid_flood_3() {
        let rains = [1, 2, 0, 1, 2].to_vec();
        assert_eq!(vec![0; 0], Solution::avoid_flood(rains));
    }
}
