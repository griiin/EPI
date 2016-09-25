fn swap(mut v: &mut Vec<i32>, a: usize, b: usize) {
    let tmp = v[a];
    v[a] = v[b];
    v[b] = tmp;
}

fn re_arrange(mut v: &mut Vec<i32>, ip: usize) {
    let p = v[ip];
    swap(&mut v, 0, ip);
    let mut j = 1;
    for i in 1..v.len() {
        if v[i] < p {
            if i != j {
                swap(&mut v, j, i);
            }
            j += 1;
        }
    }
    swap(&mut v, 0, j - 1);
}

fn main() {
    let mut v = vec![5,6,12,1,23,55,99,0,15,18];
    re_arrange(&mut v, 8);
    println!("{:?}", v);
}
