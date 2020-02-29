use std::io;
use std::io::BufRead;

pub mod data_structures {
    pub struct FenwickTree {
        data: Vec<i64>,
    }

    impl FenwickTree {
        pub fn new(size: usize) -> FenwickTree {
            let arr_size = vec![0; size];
            return FenwickTree { data: arr_size };
        }

        fn get_lsb(index: i64) -> i64 {
            return index & (-index);
        }

        /// Find the sum of the array from index [0...(index - 1)].
        pub fn get_sum(&self, index: usize) -> i64 {
            let mut sum: i64 = 0;
            let mut cur_index: usize = index;
            while cur_index > 0 {
                sum += self.data[cur_index];
                let lsb: usize = FenwickTree::get_lsb(cur_index as i64) as usize;
                cur_index -= lsb;
            }
            return sum;
        }

        /// Update a node and all ranges that contain this node.
        pub fn update(&mut self, index: usize, value: i64) {
            let mut cur_index = index;
            while cur_index < self.data.len() {
                self.data[cur_index] += value;
                let lsb: usize = FenwickTree::get_lsb(cur_index as i64) as usize;
                cur_index += lsb;
            }
        }
    }
}

struct Operation {
    opType: i64,
    op1: i64,
    op2: i64,
}

fn main() {
    let mut operations: Vec<Operation> = Vec::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buf = String::new();

    reader.read_line(&mut buf);

    let iter: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let N = iter[0] + 1;
    let M = iter[1];

    for _i in 0..M {
        let mut line = String::new();
        reader.read_line(&mut line);
        let iter: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (t, i, v) = (iter[0], iter[1], iter[2]);
        let operation = Operation {
            opType: t,
            op1: i,
            op2: v,
        };
        operations.push(operation);
    }

    let mut fenwick_tree = data_structures::FenwickTree::new(N);

    for op in operations.iter() {
        if op.opType == 1 {
            fenwick_tree.update(op.op1 as usize, op.op2 as i64);
        } else if op.opType == 2 {
            let l = fenwick_tree.get_sum((op.op1 - 1) as usize);
            let r = fenwick_tree.get_sum(op.op2 as usize);
	    println!("{}", r - l);
        }
    }
}
