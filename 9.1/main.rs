struct MaxStack {
    v: Vec<i32>,
    max: Vec<i32>,
}

impl MaxStack {
    fn new() -> MaxStack {
        let mut max = Vec::new();
        max.push(std::i32::MIN);
        MaxStack {
            v: Vec::new(),
            max: max,
        }
    }

    fn max(&mut self) -> i32 {
        self.max[self.max.len() - 1]
    }

    fn push(&mut self, n: i32) -> &mut MaxStack {
        if n >= self.max() {
            self.max.push(n);
        }
        self.v.push(n);
        self
    }

    fn pop(&mut self) -> i32 {
        let p = self.v.pop().unwrap();
        if p == self.max() {
            self.max.pop();
        }
        p
    }
}

fn main() {
    let mut max_stack = MaxStack::new();
    max_stack.push(1).push(5).push(10).push(9).push(12);
    println!("{}", max_stack.max());
    max_stack.pop();
    println!("{}", max_stack.max());
}
