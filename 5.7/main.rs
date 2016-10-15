fn pow(x: f32, y: i32) {
    let mut x = x;
    let mut result = 1.0;
    let mut power = y;
    if y < 0 {
        power = -power;
        x = 1.0 / x;
    }
    while power > 0 {
        if power & 1 != 0 {
            result *= x;
        }
        x *= x;
        power >>= 1;
    }
    println!("{}", result);
}

fn main() {
    pow(2.0, 4);
    pow(1.1, 21);
}
