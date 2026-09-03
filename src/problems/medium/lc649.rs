#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let (mut rad_dequeue, mut dir_dequeue) = senate.as_bytes().iter().enumerate().fold(
            (VecDeque::new(), VecDeque::new()),
            |(mut rad_dq, mut dir_dq), (i, member)| {
                match member {
                    b'R' => rad_dq.push_back(i),
                    b'D' => dir_dq.push_back(i),
                    _ => {}
                }
                (rad_dq, dir_dq)
            },
        );
        let mut n = senate.len();

        loop {
            match (rad_dequeue.pop_front(), dir_dequeue.pop_front()) {
                (_, None) => return "Radiant".to_string(),
                (None, _) => return "Dire".to_string(),
                (Some(r_id), Some(d_id)) => {
                    if r_id < d_id {
                        rad_dequeue.push_back(n);
                    } else {
                        dir_dequeue.push_back(n);
                    }
                    n += 1;
                }
            }
        }
    }
}
