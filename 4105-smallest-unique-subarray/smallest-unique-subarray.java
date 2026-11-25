class Solution {
    public int smallestUniqueSubarray(int[] nums) {
        int n = nums.length;
        int low = 1, high = n, ans = n;

        while (low <= high) {
            int mid = low + (high - low) / 2;

            if (check(nums, mid)) {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        return ans;
    }

    private boolean check(int[] nums, int len) {
        int n = nums.length;
        long base = 19, mod = 1000000007L;
        long[] pow = new long[len];
        pow[0] = 1;

        for (int i = 1; i < len; i++)
            pow[i] = (pow[i - 1] * base) % mod;

        HashMap<Long, Integer> map = new HashMap<>();
        long hash = 0;

        for (int i = 0; i < len; i++)
            hash = (hash * base + nums[i]) % mod;

        map.put(hash, 1);

        for (int i = 1; i <= n - len; i++) {
            hash = (hash - (pow[len - 1] * nums[i - 1]) % mod + mod) % mod;
            hash = (hash * base + nums[i + len - 1]) % mod;
            map.put(hash, map.getOrDefault(hash, 0) + 1);
        }

        for (int count : map.values())
            if (count == 1)
                return true;

        return false;
    }
}