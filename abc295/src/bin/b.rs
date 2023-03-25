use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        mut board: [Chars; r],
    }
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];

    let mut queue = VecDeque::new();
    for i in 0..r {
        for j in 0..c {
            let cell = board[i][j];
            if cell.is_numeric() {
                let power = cell.to_digit(10).unwrap() as usize;
                queue.push_back((i, j, power));
            }
        }
    }

    while let Some((x, y, p)) = queue.pop_front() {
        board[x][y] = '.';
        for i in 0..4 {
            let nx = (x as i32 + dx[i]) as usize;
            let ny = (y as i32 + dy[i]) as usize;
            if nx < r && ny < c {
                if p >= 1 {
                    queue.push_back((nx, ny, p - 1));
                }
            }
        }
    }

    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}
