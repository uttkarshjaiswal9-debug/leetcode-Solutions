import java.util.Arrays;

class Solution {
    public int[] deckRevealedIncreasing(int[] deck) {
        int n = deck.length;
        Arrays.sort(deck);
        int[] res = new int[n];
        boolean skip = false;
        for (int i = 0, j = 0; i < n; j = (j + 1) % n) {
            if (res[j] == 0) {
                if (!skip) res[j] = deck[i++];
                skip = !skip;
            }
        }
        return res;
    }
}