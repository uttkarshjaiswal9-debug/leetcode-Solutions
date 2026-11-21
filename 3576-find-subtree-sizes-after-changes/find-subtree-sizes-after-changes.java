class Solution {
    public int[] findSubtreeSizes(int[] parent, String s) {
        int n = s.length();
        List<Integer>[] g = new ArrayList[n];
        List<Integer>[] d = new ArrayList[26];
        for (int i = 0; i < n; i++) g[i] = new ArrayList<>();
        for (int i = 0; i < 26; i++) d[i] = new ArrayList<>();
        for (int i = 1; i < n; i++) g[parent[i]].add(i);
        int[] ans = new int[n];
        dfs(0, -1, g, d, s.toCharArray(), ans);
        return ans;
    }

    private void dfs(int i, int fa, List<Integer>[] g, List<Integer>[] d, char[] s, int[] ans) {
        ans[i] = 1;
        int idx = s[i] - 'a';
        d[idx].add(i);
        for (int j : g[i]) dfs(j, i, g, d, s, ans);
        int k = d[idx].size() > 1 ? d[idx].get(d[idx].size() - 2) : fa;
        if (k != -1) ans[k] += ans[i];
        d[idx].remove(d[idx].size() - 1);
    }
}