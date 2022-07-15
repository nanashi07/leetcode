// # 6. Zigzag Conversion
// https://leetcode.com/problems/zigzag-conversion/
struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            s
        } else {
            let len = s.len();
            let l = (num_rows * 2 - 2) as usize;

            let mut results: Vec<String> = Vec::new();
            for _ in 0..num_rows {
                results.push(String::new());
            }

            for n in 0..=(len / l) {
                for row_index in 0..=(l / 2) {
                    if row_index == 0 {
                        let shift = 0;
                        let pos = l * n + shift;
                        if pos < len {
                            results[row_index].push_str(&s[pos..=(pos)]);
                        }
                    } else if row_index == l / 2 {
                        let shift = l / 2;
                        let pos = l * n + shift;
                        if pos < len {
                            results[row_index].push_str(&s[pos..=(pos)]);
                        }
                    } else {
                        let shift = row_index;
                        let pos = l * n + shift;
                        if pos < len {
                            results[row_index].push_str(&s[pos..=(pos)]);
                        }

                        let shift = l - row_index;
                        let pos = l * n + shift;
                        if pos < len {
                            results[row_index].push_str(&s[pos..=(pos)]);
                        }
                    }
                }
            }

            results
                .iter_mut()
                .reduce(|a, b| {
                    a.push_str(b);
                    a
                })
                .unwrap()
                .to_owned()
        }
    }
}

// P   A   H   N
// A P L S I I G
// Y   I   R

// r = 3, l = (r + (r-2)) = 4
// 0   4   8     12               => l * n
// 1 3 5 7 9  11 13               => l * n + 1, l * n + 3
// 2   6   10                     => l * n + 2

// ----------------

// P     I    N
// A   L S  I G
// Y A   H R
// P     I

// r = 4, l = (r + (r-2)) = 6
// 0     6       12               => l * n
// 1   5 7    11 13               => l * n + 1, l * n + 5
// 2 4   8 10                     => l * n + 2, l * n + 4
// 3     9                        => l * n + 3

// r = 5, l = (r + (r-2)) = 8
// 00          08          16     => l * n
// 01       07 09       15 17     => l * n + 1, l * n + 7
// 02    06    10    14    18     => l * n + 2, l * n + 6
// 03 05       11 13       19 21  => l * n + 3, l * n + 5
// 04          12          20     => l * n + 4

#[test]
fn test_convert() {
    let s = "PAYPALISHIRING";
    let num_rows = 3;
    let result = Solution::convert(s.to_owned(), num_rows);
    assert_eq!(
        "PAHNAPLSIIGYIR".to_owned(),
        result,
        "input, s = {}, num_rows = {}",
        s,
        num_rows
    );

    let s = "PAYPALISHIRING";
    let num_rows = 4;
    let result = Solution::convert(s.to_owned(), num_rows);
    assert_eq!(
        "PINALSIGYAHRPI".to_owned(),
        result,
        "input, s = {}, num_rows = {}",
        s,
        num_rows
    );

    let s = "A";
    let num_rows = 1;
    let result = Solution::convert(s.to_owned(), num_rows);
    assert_eq!(
        "A".to_owned(),
        result,
        "input, s = {}, num_rows = {}",
        s,
        num_rows
    );
}
