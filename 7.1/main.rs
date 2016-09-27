fn to_int(s: &str) -> i32 {
    let mut i = s.len();
    let mut c = 0;
    let mut r = 0;
    let base = '0' as i32;
    while i > 0 {
        if let Some(d) = s.chars().nth(i - 1) {
            if i == 1 && d == '-' {
                r *= -1;
            } else {
                let nb = (d as i32) - base;
                r += nb * (10 as i32).pow(c);
            }
            i -= 1;
            c += 1;
        }
    }
    r
}

fn to_str(i: i32) -> String {
    let mut i = i;
    let base = '0' as i32;
    let mut v = Vec::new();
    let neg = i < 0;
    if neg {
        i *= -1;
    }
    while i > 0 {
        let r = i % 10;
        v.push(((r + base) as u8) as char);
        i /= 10;
    }
    if neg {
        v.push('-');
    }
    let mut s = String::new();
    v.reverse();
    for c in v {
        s.push(c);
    }
    s
}

fn main() {
    println!("{:?}", to_int("10"));
    println!("{:?}", to_int("123"));
    println!("{:?}", to_int("999"));
    println!("{:?}", to_int("-999"));
    println!("{:?}", to_str(10));
    println!("{:?}", to_str(123));
    println!("{:?}", to_str(999));
    println!("{:?}", to_str(-999));
}
