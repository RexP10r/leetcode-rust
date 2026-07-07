#![allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    idx: usize,
    list: Vec<i32>,
}

impl NestedIterator {
    fn flatten_nested_list(nested_list: Vec<NestedInteger>, res_vec: &mut Vec<i32>) {
        for val in nested_list {
            match val {
                NestedInteger::Int(n) => res_vec.push(n),
                NestedInteger::List(some_vec) => Self::flatten_nested_list(some_vec, res_vec),
            }
        }
    }
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            idx: 0,
            list: {
                let mut flattened_vec = Vec::with_capacity(nested_list.len());
                Self::flatten_nested_list(nested_list, &mut flattened_vec);
                flattened_vec
            },
        }
    }

    fn next(&mut self) -> i32 {
        let res = self.list[self.idx];
        self.idx += 1;
        res
    }

    fn has_next(&self) -> bool {
        self.idx < self.list.len()
    }
}
