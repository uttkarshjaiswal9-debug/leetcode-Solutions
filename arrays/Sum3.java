class Solution {
    public List<List<Integer>> threeSum(int[] a) {
        Set<List<Integer>> set = new HashSet<>();

        for (int i = 0; i < a.length; i++) {
            Set<Integer> s = new HashSet<>();
            for (int j = i + 1; j < a.length; j++) {
                int x = -a[i] - a[j];
                if (s.contains(x)) {
                    List<Integer> t = Arrays.asList(a[i], a[j], x);
                    Collections.sort(t);
                    set.add(t);
                }
                s.add(a[j]);
            }
        }
        return new ArrayList<>(set);
    }
}
