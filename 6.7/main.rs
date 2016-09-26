use std::cmp;

fn compute_differences(a: &Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..(a.len() - 1) {
        v.push(a[i + 1] - a[i]);
    }
    v
}

fn main() {
    let prices = vec![310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
    let differences = compute_differences(&prices);
    let mut max_to_there = 0;
    let mut max = 0;
    for d in differences {
        max_to_there = cmp::max(d, d + max_to_there);
        max = cmp::max(max, max_to_there);
    }
    println!("max: {}", max);
}
