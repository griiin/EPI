fn is_bst(v: &Vec<i32>, i: usize) -> bool {
    if i > v.len() - 1 || v[i] == -1 {
        return true;
    }
    let left_i = (i * 2) + 1;
    let right_i = (i * 2) + 2;
    if left_i < v.len() && v[left_i] != -1 {
        if v[left_i] > v[i] {
            return false;
        }
    }
    if right_i < v.len() && v[right_i] != -1 {
        if v[right_i] < v[i] {
            return false;
        }
    }
    is_bst(v, (i * 2) + 1) && is_bst(v, (i * 2) + 2)
}

fn main() {
    let bst = vec![19,7,43,3,11,23,47,2,5,-1,17,-1,37,-1,53];
    let r = is_bst(&bst, 0);
    println!("{}", r);
}
