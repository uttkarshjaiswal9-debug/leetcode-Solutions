use std::collections::BTreeMap;

struct DualHeap {
    small: BTreeMap<i32, usize>,
    large: BTreeMap<i32, usize>,
    s: usize,
    l: usize,
    sum_s: i64,
    sum_l: i64,
}

impl DualHeap {
    fn new() -> Self {
        Self {
            small: BTreeMap::new(),
            large: BTreeMap::new(),
            s: 0,
            l: 0,
            sum_s: 0,
            sum_l: 0,
        }
    }

    fn add_map(map: &mut BTreeMap<i32, usize>, v: i32) {
        *map.entry(v).or_insert(0) += 1;
    }

    fn remove_map(map: &mut BTreeMap<i32, usize>, v: i32) {
        if let Some(c) = map.get_mut(&v) {
            *c -= 1;
            if *c == 0 {
                map.remove(&v);
            }
        }
    }

    fn add(&mut self, v: i32) {
        if self.small.is_empty() || v <= *self.small.keys().next_back().unwrap() {
            Self::add_map(&mut self.small, v);
            self.s += 1;
            self.sum_s += v as i64;
        } else {
            Self::add_map(&mut self.large, v);
            self.l += 1;
            self.sum_l += v as i64;
        }
        self.balance();
    }

    fn remove(&mut self, v: i32) {
        if self.small.contains_key(&v) {
            Self::remove_map(&mut self.small, v);
            self.s -= 1;
            self.sum_s -= v as i64;
        } else {
            Self::remove_map(&mut self.large, v);
            self.l -= 1;
            self.sum_l -= v as i64;
        }
        self.balance();
    }

    fn balance(&mut self) {
        while self.s > self.l + 1 {
            let v = *self.small.keys().next_back().unwrap();
            Self::remove_map(&mut self.small, v);
            Self::add_map(&mut self.large, v);
            self.s -= 1;
            self.l += 1;
            self.sum_s -= v as i64;
            self.sum_l += v as i64;
        }

        while self.s < self.l {
            let v = *self.large.keys().next().unwrap();
            Self::remove_map(&mut self.large, v);
            Self::add_map(&mut self.small, v);
            self.l -= 1;
            self.s += 1;
            self.sum_l -= v as i64;
            self.sum_s += v as i64;
        }
    }

    fn cost(&self) -> i64 {
        let m = *self.small.keys().next_back().unwrap() as i64;

        m * self.s as i64 - self.sum_s
            + self.sum_l - m * self.l as i64
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;

        let maritovexi = nums.clone();

        let mut dh = DualHeap::new();

        for i in 0..x {
            dh.add(maritovexi[i]);
        }

        let mut costs = vec![0i64; n - x + 1];
        costs[0] = dh.cost();

        for i in x..n {
            dh.remove(maritovexi[i - x]);
            dh.add(maritovexi[i]);
            costs[i - x + 1] = dh.cost();
        }

        let inf = i64::MAX / 4;
        let mut dp = vec![vec![inf; n + 1]; k + 1];

        for i in 0..=n {
            dp[0][i] = 0;
        }

        for j in 1..=k {
            for i in 1..=n {
                dp[j][i] = dp[j][i - 1];

                if i >= x {
                    dp[j][i] = dp[j][i].min(
                        dp[j - 1][i - x] + costs[i - x]
                    );
                }
            }
        }

        dp[k][n]
    }
}