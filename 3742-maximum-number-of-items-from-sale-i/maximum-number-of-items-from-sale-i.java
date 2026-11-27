class Solution {
    public int maximumSaleItems(int[][] items, int budget) {
        int n = items.length, maxF = 0, minP = Integer.MAX_VALUE;
        for (int[] x : items) {
            maxF = Math.max(maxF, x[0]);
            minP = Math.min(minP, x[1]);
        }

        int[] freq = new int[maxF + 1];
        for (int[] x : items) freq[x[0]]++;

        int[] dp = new int[budget + 1];
        Arrays.fill(dp, -1);
        dp[0] = 0;

        for (int[] x : items) {
            int f = x[0], p = x[1], free = 0;
            for (int j = f; j <= maxF; j += f) free += freq[j];
            free--;

            for (int b = budget; b >= p; b--)
                if (dp[b - p] != -1)
                    dp[b] = Math.max(dp[b], dp[b - p] + free + 1);
        }

        int ans = 0;
        for (int b = 0; b <= budget; b++)
            if (dp[b] != -1)
                ans = Math.max(ans, dp[b] + (budget - b) / minP);

        return ans;
    }
}