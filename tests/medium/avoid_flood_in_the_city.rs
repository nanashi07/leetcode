// # 1488. Avoid Flood in The City
// https://leetcode.com/problems/avoid-flood-in-the-city/

use std::collections::{BTreeSet, HashMap};

struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut result = vec![1; n]; // Default dry days to 1
        let mut full_lakes: HashMap<i32, usize> = HashMap::new(); // lake -> day it was filled
        let mut dry_days: BTreeSet<usize> = BTreeSet::new(); // Available dry days

        for i in 0..n {
            if rains[i] == 0 {
                // Dry day - we'll decide later which lake to dry
                dry_days.insert(i);
            } else {
                let lake = rains[i];
                result[i] = -1; // Rainy day

                // Check if this lake is already full
                if let Some(&prev_day) = full_lakes.get(&lake) {
                    // Lake is full! We need to find a dry day between prev_day and now
                    // to dry this lake before it floods

                    // Find the first dry day after prev_day
                    let dry_day = dry_days.range(prev_day..).next().copied();

                    if let Some(day) = dry_day {
                        // Use this dry day to dry the lake
                        result[day] = lake;
                        dry_days.remove(&day);
                        // Update the lake's full status to current day
                        full_lakes.insert(lake, i);
                    } else {
                        // No dry day available - flood!
                        return vec![];
                    }
                } else {
                    // Lake wasn't full, now it is
                    full_lakes.insert(lake, i);
                }
            }
        }

        result
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
