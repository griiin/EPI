fn spiral_display(grid: Vec<Vec<i32>>, size: usize, d: usize) {
    if d == size / 2 {
        if size % 2 == 1 {
            println!("{}", grid[d][d]);
        } else {
            println!("END");
        }
        return;
    }
    // up
    for i in (d)..(size - d) {
        print!("{}, ", grid[d][i]);
    }
    // right
    for i in (d + 1)..(size - d) {
        print!("{}, ", grid[i][size - d - 1]);
    }
    // botton
    for i in ((d)..(size - d - 1)).rev() {
        print!("{}, ", grid[size - d - 1][i]);
    }
    // left
    for i in ((d + 1)..(size - d - 1)).rev() {
        print!("{}, ", grid[i][d]);
    }
    spiral_display(grid, size, d + 1);
}

fn main() {
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    spiral_display(grid, 4, 0);
}
