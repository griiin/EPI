// O(n * (ln word_size)) = O(n * 6)
fn is_even(w: u64) -> bool {
    let mut x = w;
    x ^= x >> 32;
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    (x & 0b1) == 0
}

fn are_even(v: Vec<u64>) -> bool {
    let mut r = true;
    for i in v {
        r = !(is_even(i) ^ r);
    }
    r
}

fn main() {
    let o = 0b0000000010000000000000000000000000001000000000000100000000000000u64;
    let e = 0b0000000010000000000000000010000000001000000000000100000000000000u64;
    let mut v = Vec::new();
    for _ in 0..10000000 {
        v.push(e);
        v.push(o);
    }
    println!("{:?}", are_even(v));
}
