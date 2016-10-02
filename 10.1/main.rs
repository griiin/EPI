use std::cmp;

enum Tree {
    Node(Box<Tree>, Box<Tree>),
    Leaf
}

fn is_balanced(t: Tree) -> bool {
    fn inner_loop(t: Tree) -> i32 {
        match t {
            Tree::Leaf => 0,
            Tree::Node(a, b) => {
                match (*a, *b) {
                    (Tree::Leaf, Tree::Leaf) => 1,
                    (l, r) => {
                        let lr = inner_loop(l);
                        let rr = inner_loop(r);
                        let max = cmp::max(lr, rr);
                        let min = cmp::min(lr, rr);
                        if lr == -1 || rr == -1 || max - min > 1 {
                            -1
                        } else {
                            1 + max
                        }
                    },
                }
            },
        }
    }
    inner_loop(t) != -1
}

fn main() {
    let good_tree = Tree::Node(
                Box::new(Tree::Node(
                    Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
                    Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf)))
                )),
                Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf)))
            );
    println!("{:?}", is_balanced(good_tree));
    let bad_tree = Tree::Node(
                Box::new(Tree::Node(
                    Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
                    Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf)))
                )),
                Box::new(Tree::Leaf)
            );
    println!("{:?}", is_balanced(bad_tree));
}
