#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn invert(l: List<i32>, nl: List<i32>) -> List<i32> {
    match l {
        List::Nil => nl,
        List::Cons(x, xs) => {
            invert(*xs, List::Cons(x, Box::new(nl)))
        }
    }
}

fn main() {
    let list: List<i32> = List::Cons(2, Box::new(List::Cons(5, Box::new(List::Cons(7, Box::new(List::Nil))))));
    let result = invert(list, List::Nil);
    println!("{:?}", result);
}
