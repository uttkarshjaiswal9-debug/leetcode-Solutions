use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    end_time: i64,
    server_id: usize,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.end_time.cmp(&self.end_time)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut available: BTreeSet<usize> = (0..k).collect();
        let mut busy: BinaryHeap<Task> = BinaryHeap::new();
        let mut count = vec![0; k];

        for i in 0..arrival.len() {
            let arr = arrival[i] as i64;
            let l = load[i] as i64;

            while let Some(task) = busy.peek() {
                if task.end_time <= arr {
                    let task = busy.pop().unwrap();
                    available.insert(task.server_id);
                } else {
                    break;
                }
            }

            if available.is_empty() {
                continue;
            }

            let target = i % k;
            let assigned = if let Some(&server) = available.range(target..).next() {
                server
            } else {
                *available.iter().next().unwrap()
            };

            available.remove(&assigned);
            busy.push(Task {
                end_time: arr + l,
                server_id: assigned,
            });
            count[assigned] += 1;
        }

        let max_requests = *count.iter().max().unwrap_or(&0);
        let mut result = Vec::new();
        for i in 0..k {
            if count[i] == max_requests {
                result.push(i as i32);
            }
        }

        result
    }
}