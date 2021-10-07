use proconio::input;

fn main() {
    input! {
        s: i32,
        w: i32,
    }

    if s > w {
        println!("safe");
        return;
    }
    println!("unsafe");
}