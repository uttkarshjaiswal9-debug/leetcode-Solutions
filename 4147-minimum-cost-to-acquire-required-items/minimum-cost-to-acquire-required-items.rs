impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let c1 = cost1 as i64;
        let c2 = cost2 as i64;
        let cb = cost_both as i64;
        let n1 = need1 as i64;
        let n2 = need2 as i64;

        let common = n1.min(n2);
        let rem1 = n1 - common;
        let rem2 = n2 - common;

        let cost_common = common * cb.min(c1 + c2);
        let cost_rem1 = rem1 * cb.min(c1);
        let cost_rem2 = rem2 * cb.min(c2);

        cost_common + cost_rem1 + cost_rem2
    }
}