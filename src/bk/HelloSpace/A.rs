use proconio::input;

fn main() {
    input!{
        s: String,
    }
 
    let mut ans = 0;
    for i in 0..=(s.len() - 4) {
        if &s[i..i+4] == "ZONe" {
            ans += 1;
        }
    }
    println!("{}", ans);
}