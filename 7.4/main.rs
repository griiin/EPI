fn remove(a: [char; 4]) -> [char; 4] {
    let mut a = a;
    let mut i = 0;
    let mut j = 0;
    while i < 4 {
        if a[i] != 'b' {
            a[j] = a[i];
            j += 1;
        }
        i += 1;
    }
    while j < 4 {
        a[j] = ' ';
        j += 1;
    }
    a
}

fn replace(a: [char; 4]) -> [char; 4] {
    let mut a = a;
    let mut i = 4;
    let mut j = 4;
    while i > 0 {
        if a[i - 1] == 'a' {
            a[j - 1] = 'b';
            j -= 1;
            a[j - 1] = 'b';
            j -= 1;
        }
        else if a[i - 1] != ' ' {
            a[j - 1] = a[i - 1];
            j -= 1;
        }
        i -= 1;
    }
    a
}

fn main() {
    let a = ['a', 'b', 'c', 'd'];
    println!("{:?}", replace(remove(a)));
}
