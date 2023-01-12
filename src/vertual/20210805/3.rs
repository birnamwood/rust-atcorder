
use proconio::input;

fn main() {
    input!{
        n: i128,
    }
    let mut power:i128 = 1;

    for i in 1..n + 1 {
        power = power * i;
        power = power % (1000000000 + 7);
    }

    println!("{}", power);
}