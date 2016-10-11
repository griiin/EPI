use std::collections::HashMap;

enum Status {
    End,
    Wall,
    Empty,
}

fn print_result(graph: &HashMap<(i32, i32), i32>, start:(i32, i32), end:(i32, i32)) {
    println!("Finished!");
    let mut result = Vec::new();
    result.push(end);
    let mut current = end;
    let mut explored = HashMap::new();
    loop {
        let (x, y) = current;
        let mut min_value = 0;
        let mut min = (-1, -1);
        let u = ((x - 1), y);
        let d = ((x + 1), y);
        let l = (x, (y - 1));
        let r = (x, (y + 1));
        if u == start || d == start || l == start || r == start {
            break;
        }
        explored.insert(current, true);
        if x > 0 && !explored.contains_key(&u) {
            if let Some(v) = graph.get(&u) {
                if *v != 0 && (min_value == 0 || min_value >= *v) {
                    min = u;
                    min_value = *v;
                }
            }
        }
        if x < 3 && !explored.contains_key(&d) {
            if let Some(v) = graph.get(&d) {
                if *v != 0 && (min_value == 0 || min_value >= *v) {
                    min = d;
                    min_value = *v;
                }
            }
        }
        if y > 0 && !explored.contains_key(&l) {
            if let Some(v) = graph.get(&l) {
                if *v != 0 && (min_value == 0 || min_value >= *v) {
                    min = l;
                    min_value = *v;
                }
            }
        }
        if y < 3 && !explored.contains_key(&r) {
            if let Some(v) = graph.get(&r) {
                if *v != 0 && (min_value == 0 || min_value >= *v) {
                    min = r;
                }
            }
        }
        current = min;
        result.push(min);
    }
    result.push(start);
    println!("{:?}", result);
}

fn main() {
    let mut maze = HashMap::new();
    maze.insert((0, 0), Status::Empty);
    maze.insert((0, 1), Status::Empty);
    maze.insert((0, 2), Status::Empty);
    maze.insert((0, 3), Status::End);
    maze.insert((1, 0), Status::Empty);
    maze.insert((1, 1), Status::Empty);
    maze.insert((1, 2), Status::Wall);
    maze.insert((1, 3), Status::Wall);
    maze.insert((2, 0), Status::Wall);
    maze.insert((2, 1), Status::Empty);
    maze.insert((2, 2), Status::Empty);
    maze.insert((2, 3), Status::Empty);
    maze.insert((3, 0), Status::Empty);
    maze.insert((3, 1), Status::Empty);
    maze.insert((3, 2), Status::Empty);
    maze.insert((3, 3), Status::Empty);
    let mut stack:Vec<(i32, i32, i32)> = Vec::new();
    let mut explored = HashMap::new();
    let start = (3, 0);
    stack.push((start.0, start.1, 0));
    while let Some((x, y, level)) = stack.pop() {
        if explored.contains_key(&(x, y)) {
            continue;
        }
        match maze.get(&(x, y)) {
            Some(status) => {
                match *status {
                    Status::Wall => {
                        continue;
                    },
                    Status::End => {
                        print_result(&explored, start, (x, y));
                        return;
                    },
                    Status::Empty => ()
                }
            },
            None => (),
        }
        if x > 0 {
            stack.push(((x - 1), y, level + 1));
        }
        if x < 3 {
            stack.push(((x + 1), y, level + 1));
        }
        if y > 0 {
            stack.push((x, (y - 1), level + 1));
        }
        if y < 3 {
            stack.push((x, (y + 1), level + 1));
        }
        explored.insert((x, y), level);
    }
}
