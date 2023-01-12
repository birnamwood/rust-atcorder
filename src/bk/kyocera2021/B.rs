use proconio::input;

fn main() {
    input!{
        n: i64,
        k: i64,
    }
    let mut nn: i64 = n;

    for _i in 0..k {
        if nn % 200 == 0 {
            nn = nn / 200;
        } else {
            let m = nn.to_string() + "200";
            nn = m.parse().unwrap();
        }
    }
    println!("{}", nn);
}