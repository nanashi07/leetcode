// 1948. Delete Duplicate Folders in System
// https://leetcode.com/problems/delete-duplicate-folders-in-system/

struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    serial: String,                  // current node structure's serialized representation
    children: HashMap<String, Trie>, // current node's child nodes
}

impl Solution {
    // https://leetcode.com/problems/delete-duplicate-folders-in-system/editorial/
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::default(); // root node
                                        // build a trie tree
        for path in paths {
            let mut cur = &mut root;
            for node in path {
                cur = cur.children.entry(node.clone()).or_default();
            }
        }

        let mut freq = HashMap::new(); // hash table records the occurrence times of each serialized representation

        // post-order traversal based on depth-first search, calculate the serialized representation of each node structure
        fn construct(node: &mut Trie, freq: &mut HashMap<String, usize>) {
            if node.children.is_empty() {
                return; // if it is a leaf node, no operation is needed.
            }

            let mut v = Vec::new();
            for (folder, child) in node.children.iter_mut() {
                construct(child, freq);
                v.push(format!("{}({})", folder, child.serial));
            }

            v.sort();
            node.serial = v.join("");
            *freq.entry(node.serial.clone()).or_default() += 1;
        }
        construct(&mut root, &mut freq);
        let mut ans = Vec::new();
        let mut path = Vec::new();

        // operate the trie, delete duplicate folders
        fn operate(
            node: &Trie,
            freq: &HashMap<String, usize>,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if freq.get(&node.serial).unwrap_or(&0) > &1 {
                return; // if the serialization representation appears more than once, it needs to be deleted
            }

            if !path.is_empty() {
                ans.push(path.clone());
            }

            for (folder, child) in &node.children {
                path.push(folder.clone());
                operate(child, freq, path, ans);
                path.pop();
            }
        }
        operate(&root, &freq, &mut path, &mut ans);

        ans
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
        let mut output = [vec!["d"], vec!["d", "a"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        output.sort_unstable();
        let mut result = Solution::delete_duplicate_folder(paths);
        result.sort_unstable();
        assert_eq!(output, result);
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
        let mut output = [vec!["c"], vec!["c", "b"], vec!["a"], vec!["a", "b"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        output.sort_unstable();
        let mut result = Solution::delete_duplicate_folder(paths);
        result.sort_unstable();
        assert_eq!(output, result);
    }

    #[test]
    fn test_delete_duplicate_folder_3() {
        let paths = [vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        let mut output = [vec!["c"], vec!["c", "d"], vec!["a"], vec!["a", "b"]]
            .iter()
            .map(|l| l.iter().map(|&s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        output.sort_unstable();
        let mut result = Solution::delete_duplicate_folder(paths);
        result.sort_unstable();
        assert_eq!(output, result);
    }
}
