use std::time::Instant;

// # 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water/
struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        while left < right {
            let v1 = height[left];
            let v2 = height[right];

            max = max.max((right - left) as i32 * v1.min(v2));
            if v1 > v2 {
                right = right - 1;
            } else {
                left = left + 1;
            }
        }
        max
    }
}

#[test]
fn test_max_area() {
    let time = Instant::now();
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(height.clone());
    println!("coset time: {:?}", time.elapsed());
    assert_eq!(49, result, "hight: {:?}", &height);

    let time = Instant::now();
    let height = vec![1, 1];
    let result = Solution::max_area(height.clone());
    println!("coset time: {:?}", time.elapsed());
    assert_eq!(1, result, "hight: {:?}", &height);

    let time = Instant::now();
    let height = read_case_01();
    let result = Solution::max_area(height.clone());
    println!("coset time: {:?}", time.elapsed());
    assert_eq!(705634720, result);
}

fn read_case_01() -> Vec<i32> {
    let path = "testcase/medium/container_with_most_water/testcase01.txt";
    let content = std::fs::read_to_string(path).unwrap();
    content
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
