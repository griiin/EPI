fn main() {
    let a = vec![2,3,3,5,5,6,7,7,8,12];
    let b = vec![5,5,6,8,8,9,10,10];
    let mut r = Vec::new();
    let mut i_a = 0;
    let mut i_b = 0;
    while a.len() > i_a && b.len() > i_b {
        let e_a = a[i_a];
        let e_b = b[i_b];
        if e_a == e_b {
            r.push(e_a);
            while a[i_a] == b[i_b] && a[i_a] == e_a {
                i_a += 1;
                i_b += 1;
            }
        } else if e_a > e_b {
            i_b += 1;
        } else {
            i_a += 1;
        }
    }
    println!("{:?}", r);
}
