#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn merge(l1: List<i32>, l2: List<i32>) -> List<i32> {
    match (l1, l2) {
        (List::Nil, List::Nil) => List::Nil,
        (List::Nil, b) => b,
        (a, List::Nil) => a,
        (List::Cons(a, ax), List::Cons(b, bx)) => {
            if a < b {
                List::Cons(a, Box::new(merge(*ax, List::Cons(b, bx))))
            } else {
                List::Cons(b, Box::new(merge(*bx, List::Cons(a, ax))))
            }
        },
    }
}

fn main() {
    let list_1: List<i32> = List::Cons(2, Box::new(List::Cons(5, Box::new(List::Cons(7, Box::new(List::Nil))))));
    let list_2: List<i32> = List::Cons(3, Box::new(List::Cons(11, Box::new(List::Nil))));
    let list_3 = merge(list_1, list_2);
    println!("{:?}", list_3);
}
