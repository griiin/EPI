fn swap(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    println!("unpop {:?} to {:?}", a, b);
    if let Some(v) = a.pop() {
        b.push(v);
    }
}

fn anoi(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>, len: usize) {
    match len {
        0 => (),
        1 => {
            swap(b, a);
        },
        2 => {
            swap(a, c);
            swap(a, b);
            swap(c, b);
        },
        _ => {
            anoi(a, c, b, len - 1);
            swap(a, b);
            anoi(c, b, a, len - 1);
        },
    }
}

fn main() {
    let mut a:Vec<i32> = vec![6,5,4,3,2,1];
    let mut b:Vec<i32> = Vec::new();
    let mut c:Vec<i32> = Vec::new();
    // a.push(4);
    // a.push(3);
    // a.push(2);
    // a.push(1);
    let len = a.len();
    println!("==================================");
    println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c);
    println!("==================================");
    anoi(&mut a, &mut b, &mut c, len);
    println!("==================================");
    println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c);
    println!("==================================");
}
