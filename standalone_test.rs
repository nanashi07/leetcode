fn max_sum_trionic(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut dp = vec![vec![i64::MIN / 2; 3]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let val = nums[i - 1] as i64;
        dp[i][0] = dp[i - 1][0] + val;
        if i >= 2 {
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][0]) - val;
        }
        if i >= 3 {
            dp[i][2] = dp[i - 1][2].max(dp[i - 1][1]) + val;
        }
    }

    // Debug: print all dp values
    println!("DP table:");
    for i in 0..=n {
        println!("dp[{}] = [{}, {}, {}]", i, dp[i][0], dp[i][1], dp[i][2]);
    }

    // Try different return strategies
    println!("\nStrategy 1 - dp[n][2]: {}", dp[n][2]);
    println!(
        "Strategy 2 - max all: {}",
        (0..=n)
            .flat_map(|i| (0..3).map(move |j| dp[i][j]))
            .max()
            .unwrap()
    );
    println!(
        "Strategy 3 - max phase 0: {}",
        (0..=n).map(|i| dp[i][0]).max().unwrap()
    );
    println!("Strategy 4 - dp[n-1][0]: {}", dp[n - 1][0]);

    dp[n][2]
}

fn main() {
    println!("Test 1:");
    let result1 = max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]);
    println!("Result: {} (expected: -4)\n", result1);

    println!("Test 2:");
    let result2 = max_sum_trionic(vec![1, 4, 2, 7]);
    println!("Result: {} (expected: 14)", result2);
}
