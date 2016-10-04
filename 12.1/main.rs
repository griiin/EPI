fn binary_search(input: &Vec<i32>, value: i32, p: usize, q: usize) -> usize {
    if p == q {
        q
    } else if q - p == 1 {
        if input[p] == value {
            p
        } else {
            q
        }
    } else {
        let mid = ((q - p) / 2) + p;
        let value_mid = input[mid];
        if value > value_mid {
            binary_search(input, value, mid, q)
        } else if value < value_mid {
            binary_search(input, value, p, mid - 1)
        } else {
            binary_search(input, value, p, mid)
        }
    }
}

fn find_first_index(input: &Vec<i32>, value: i32) -> usize {
    binary_search(input, value, 0, input.len() - 1)
}

fn main() {
    let input = vec![-14, -10, 2, 108, 108, 243, 285, 285, 285, 401];
    println!("{}", find_first_index(&input, 108));
    println!("{}", find_first_index(&input, 285));
}
