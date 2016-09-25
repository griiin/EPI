fn swap(mut v: &mut Vec<i32>, a: usize, b: usize) {
    let tmp = v[a];
    v[a] = v[b];
    v[b] = tmp;
}

fn re_arrange(mut v: &mut Vec<i32>, ip: usize) {
    let p = v[ip];
    let mut equal = 0;
    let mut smaller = 0;
    let mut larger = v.len() - 1;
    while equal <= larger {
        if v[equal] < p {
            swap(&mut v, smaller, equal);
            smaller += 1;
            equal += 1;
        } else if v[equal] == p {
            equal += 1;
        } else {
            swap(&mut v, equal, larger);
            larger -= 1;
        }
    }
}

fn main() {
    let mut v = vec![5,6,12,1,15,23,55,99,0,15,18,15];
    re_arrange(&mut v, 9);
    println!("{:?}", v);
}
