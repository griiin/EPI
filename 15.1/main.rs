fn fill_in_order(v: &Vec<i32>, i: usize, r: &mut Vec<i32>) {
    if i > v.len() - 1 || v[i] == -1 {
        return;
    }
    let left_i = (i * 2) + 1;
    let right_i = (i * 2) + 2;
    fill_in_order(v, left_i, r);
    r.push(v[i]);
    fill_in_order(v, right_i, r);
}

fn is_in_order(v: Vec<i32>) -> bool {
    let mut last = -1;
    for c in v {
        if c < last {
            return false;
        }
        last = c;
    }
    true
}

fn main() {
    let bst = vec![19,7,43,3,11,23,47,2,5,-1,17,-1,37,-1,53];
    let mut in_order:Vec<i32> = Vec::new();
    fill_in_order(&bst, 0, &mut in_order);
    let r = is_in_order(in_order);
    println!("{:?}", r);
}
