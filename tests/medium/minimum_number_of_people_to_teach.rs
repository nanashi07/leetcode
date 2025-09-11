// # 1733. Minimum Number of People to Teach
// https://leetcode.com/problems/minimum-number-of-people-to-teach/

struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let num_people = languages.len();

        // Convert languages to HashSet for O(1) lookup
        let lang_sets: Vec<HashSet<i32>> = languages
            .into_iter()
            .map(|langs| langs.into_iter().collect())
            .collect();

        // Find friendships that cannot communicate (no common language)
        let mut broken_friendships = Vec::new();

        for friendship in &friendships {
            let person1 = (friendship[0] - 1) as usize; // Convert to 0-indexed
            let person2 = (friendship[1] - 1) as usize;

            // Check if they have any common language
            let has_common_lang = lang_sets[person1]
                .intersection(&lang_sets[person2])
                .next()
                .is_some();

            if !has_common_lang {
                broken_friendships.push((person1, person2));
            }
        }

        // If no broken friendships, no one needs to learn anything
        if broken_friendships.is_empty() {
            return 0;
        }

        let mut min_teachings = num_people as i32;

        // Try each language from 1 to n
        for lang in 1..=n {
            // Count people involved in broken friendships who don't know this language
            let mut people_to_teach = HashSet::new();

            for &(person1, person2) in &broken_friendships {
                if !lang_sets[person1].contains(&lang) {
                    people_to_teach.insert(person1);
                }
                if !lang_sets[person2].contains(&lang) {
                    people_to_teach.insert(person2);
                }
            }

            min_teachings = min_teachings.min(people_to_teach.len() as i32);
        }

        min_teachings
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_people_to_teach::Solution;

    #[test]
    fn test_minimum_teachings_1() {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = [[1, 2], [1, 3], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::minimum_teachings(n, languages, friendships));
    }

    #[test]
    fn test_minimum_teachings_2() {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = [[1, 4], [1, 2], [3, 4], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::minimum_teachings(n, languages, friendships));
    }
}
