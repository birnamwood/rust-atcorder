use proconio::input;

fn main() {
    input!{
        a: String,
    }
    let mut ans: i32 = 0;
    let mut flg: bool = false;

    for i in a.as_str().chars() {
        if i.to_string() == "R" {
            if flg == true {
                ans = ans + 1;
            } else {
                ans = 1;
            }
            flg = true;
        } else if i.to_string() == "S" {
            flg = false;
        }
    }

    println!("{:?}", ans);
}