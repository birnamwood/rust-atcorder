use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    let mut cc:i32 = c;

    while cc <= b {
        if cc >= a && cc <= b {
            println!("{}", cc);
            return;
        }
        cc = cc + c;
    }
    println!("{}", -1);
}