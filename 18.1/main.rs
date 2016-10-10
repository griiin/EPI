fn main() {
    let t = 21;
    let v = vec![2, 3, 5, 7, 11];
    for i in 0..v.len() {
        for j in i..v.len() {
            for k in j..v.len() {
                if v[i] + v[j] + v[k] == t {
                    println!("Found {} {} {} = {}", v[i], v[j], v[k], t);
                }
            }
        }
    }
}
