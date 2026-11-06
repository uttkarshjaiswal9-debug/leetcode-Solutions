impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let m = queries.len();
        let mut indexed_queries: Vec<(i32, usize)> = queries.into_iter().enumerate().map(|(i, q)| (q, i)).collect();
        indexed_queries.sort_unstable();

        let mut sorted_logs = logs;
        sorted_logs.sort_unstable_by_key(|l| l[1]);

        let mut ans = vec![0; m];
        let mut counts = vec![0; (n + 1) as usize];
        let mut active_servers = 0;
        let mut left = 0;
        let mut right = 0;

        for &(q, orig_idx) in &indexed_queries {
            let start = q - x;
            
            while right < sorted_logs.len() && sorted_logs[right][1] <= q {
                let id = sorted_logs[right][0] as usize;
                if counts[id] == 0 {
                    active_servers += 1;
                }
                counts[id] += 1;
                right += 1;
            }

            while left < sorted_logs.len() && sorted_logs[left][1] < start {
                let id = sorted_logs[left][0] as usize;
                counts[id] -= 1;
                if counts[id] == 0 {
                    active_servers -= 1;
                }
                left += 1;
            }

            ans[orig_idx] = n - active_servers;
        }

        ans
    }
}