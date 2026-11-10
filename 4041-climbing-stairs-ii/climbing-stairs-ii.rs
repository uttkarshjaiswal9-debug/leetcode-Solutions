impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let cost_i = costs[i - 1];

            if i >= 1 && dp[i - 1] != i32::MAX {
                dp[i] = dp[i].min(dp[i - 1] + cost_i + 1);
            }
            if i >= 2 && dp[i - 2] != i32::MAX {
                dp[i] = dp[i].min(dp[i - 2] + cost_i + 4);
            }
            if i >= 3 && dp[i - 3] != i32::MAX {
                dp[i] = dp[i].min(dp[i - 3] + cost_i + 9);
            }
        }

        dp[n]
    }
}