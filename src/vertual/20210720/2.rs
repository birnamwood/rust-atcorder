use proconio::input;

fn main() {
    input!{
        a: i16,
        b: i16,
    }
    let ans = (a-1) * (b-1);

    println!("{}", ans);
}