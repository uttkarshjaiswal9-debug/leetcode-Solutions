use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let n = servers.len();
        let mut available = BinaryHeap::new();
        let mut busy = BinaryHeap::new();

        for (i, &weight) in servers.iter().enumerate() {
            available.push(Reverse((weight, i as i32)));
        }

        let mut ans = Vec::new();
        let mut time = 0i64;

        for (i, &task) in tasks.iter().enumerate() {
            time = time.max(i as i64);

            while let Some(&Reverse((finish, weight, index))) = busy.peek() {
                if finish <= time {
                    busy.pop();
                    available.push(Reverse((weight, index)));
                } else {
                    break;
                }
            }

            if available.is_empty() {
                if let Some(&Reverse((finish, _, _))) = busy.peek() {
                    time = finish;
                }

                while let Some(&Reverse((finish, weight, index))) = busy.peek() {
                    if finish <= time {
                        busy.pop();
                        available.push(Reverse((weight, index)));
                    } else {
                        break;
                    }
                }
            }

            let Reverse((weight, index)) = available.pop().unwrap();

            ans.push(index);

            busy.push(Reverse((
                time + task as i64,
                weight,
                index
            )));
        }

        ans
    }
}