// # 1233. Remove Sub-Folders from the Filesystem
// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/

struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort_unstable();
        let mut output: Vec<String> = Vec::new();

        for f in folder {
            if output.is_empty() || !f.starts_with(&format!("{}/", &output[output.len() - 1])) {
                output.push(f.clone())
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::remove_sub_folders_from_the_filesystem::Solution;

    #[test]
    fn test_remove_subfolders_1() {
        let folder = ["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        let output = ["/a", "/c/d", "/c/f"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(output, Solution::remove_subfolders(folder))
    }

    #[test]
    fn test_remove_subfolders_2() {
        let folder = ["/a", "/a/b/c", "/a/b/d"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        let output = ["/a"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(output, Solution::remove_subfolders(folder))
    }

    #[test]
    fn test_remove_subfolders_3() {
        let folder = ["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        let output = ["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(output, Solution::remove_subfolders(folder))
    }
}
