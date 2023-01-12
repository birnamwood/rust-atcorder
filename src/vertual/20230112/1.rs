use proconio::input;

fn main() {
    input! {
        num: i32,
    }
    let mut ans: String = "".to_string();

    let n = num.to_string();

    for i in n.chars() {
        if &i.to_string() == "9" {
            ans = ans + "1";
        } else if &i.to_string() == "1" {
            ans = ans + "9";
        } else {
            ans = ans + &i.to_string();
        }
    }
    println!("{}", ans);
}
