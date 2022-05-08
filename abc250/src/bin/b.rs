use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }
    for i in 0..n {
        for _ in 0..a {
            let mut row = vec![];
            for j in 0..n {
                for _ in 0..b {
                    if (i + j) % 2 == 0 {
                        row.push('.');
                    } else {
                        row.push('#');
                    }
                }
            }
            println!("{}", row.iter().collect::<String>());
        }
    }
}
