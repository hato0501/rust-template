use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let answer = if a % 2 == 0 || b % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    println!("{}", answer)
}
