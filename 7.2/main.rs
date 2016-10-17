fn convert_to_number(s: &str, base: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;
    for c in s.chars().rev() {
        let c_n = (c as u32) - ('0' as u32);
        result += c_n * base.pow(i);
        i += 1;
    }
    result
}

fn convert_base(s: &str, from: u32, to: u32) -> String {
    let mut nb_base_10 = convert_to_number(s, from);
    let mut result = String::new();
    while nb_base_10 > 0 {
        let c = ('0' as u32) + (nb_base_10 % to);
        let c = c as u8;
        let c = c as char;
        result.push(c);
        nb_base_10 /= to;
    }
    result.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", convert_base("1000", 2, 8));
    println!("{}", convert_base("10", 8, 2));
}
