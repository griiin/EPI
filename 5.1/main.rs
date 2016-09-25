fn is_even(w: u64) -> bool {
    let mut r = true;
    let mut mask = 0b1u64;
    let u64_max_value = 0b10000000000000000000000000000000u64;
    loop {
        if mask == u64_max_value {
            break;
        }

        if w & mask > 0 {
            r = !r;
        }


        mask = mask << 1;
    }
    r
}

fn are_even(v: Vec<u64>) -> bool {
    let mut r = true;
    for i in v {
        r = !(is_even(i) ^ r);
    }
    r
}

fn main() {
    let o = 0b01u64;
    let e = 0b11u64;
    println!("{:?}", are_even(vec![o]));
    println!("{:?}", are_even(vec![e]));
    println!("{:?}", are_even(vec![o, o]));
    println!("{:?}", are_even(vec![o, e, o, e, o]));
}
