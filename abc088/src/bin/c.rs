use proconio::input;

fn main() {
    input! {
        c:[[i64;3];3],
    }
    println!(
        "{}",
        if (1..3)
            .into_iter()
            .any(|i| c[1][i] - c[0][i] != c[1][0] - c[0][0]
                || c[2][i] - c[1][i] != c[2][0] - c[1][0]
                || c[i][1] - c[i][0] != c[0][1] - c[0][0]
                || c[i][2] - c[i][1] != c[0][2] - c[0][1])
        {
            "No"
        } else {
            "Yes"
        }
    );
}
