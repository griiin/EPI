use std::collections::BinaryHeap;

fn merge_and_sort(input: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut max_heap = BinaryHeap::new();
    let mut indexes = vec![0; input.len()];
    let mut i = 0;
    for v in &mut input.iter() {
        max_heap.push(v[indexes[i]]);
        indexes[i] += 1;
        i += 1;
    }
    while let Some(max_element) = max_heap.pop() {
        result.push(max_element);
        i = 0;
        for v in &mut input.iter() {
            if v.len() > indexes[i] {
                let max = v[indexes[i]];
                max_heap.push(max);
                if let Some(max) = max_heap.pop() {
                    result.push(max);
                }
                indexes[i] += 1;
                i += 1;
            }
        }
    }
    result
}

fn main() {
    let mut v = vec![vec![7,5,3], vec![6,0], vec![28,6,0]];
    let r = merge_and_sort(&mut v);
    println!("{:?}", r);
}
