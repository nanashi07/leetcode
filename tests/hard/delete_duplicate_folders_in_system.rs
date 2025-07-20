// # 1948. Delete Duplicate Folders in System
// https://leetcode.com/problems/delete-duplicate-folders-in-system/

struct Solution;

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::delete_duplicate_folders_in_system::Solution;

    #[test]
    fn test_delete_duplicate_folder_1() {
        let paths = [
            vec!["a"],
            vec!["c"],
            vec!["d"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["d", "a"],
        ]
        .iter()
        .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
        let output = [vec!["d"], vec!["d", "a"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        assert_eq!(output, Solution::delete_duplicate_folder(paths));
    }

    #[test]
    fn test_delete_duplicate_folder_2() {
        let paths = [
            vec!["a"],
            vec!["c"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["a", "b", "x"],
            vec!["a", "b", "x", "y"],
            vec!["w"],
            vec!["w", "y"],
        ]
        .iter()
        .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
        let output = [vec!["c"], vec!["c", "b"], vec!["a"], vec!["a", "b"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        assert_eq!(output, Solution::delete_duplicate_folder(paths));
    }

    #[test]
    fn test_delete_duplicate_folder_3() {
        let paths = [vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        let output = [vec!["c"], vec!["c", "d"], vec!["a"], vec!["a", "b"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        assert_eq!(output, Solution::delete_duplicate_folder(paths));
    }
}
