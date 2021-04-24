use proconio::input;

fn mainbk() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    let mut same: bool = false;
    let mut v: Vec<i64> = Vec::with_capacity(n);

    for i in &a  {
        for j in &b  {
            if i == j {
                same = true;
            }
        }
            println!("{}", same);
        if same != true {
            v.push(*i)
        }
        same = false;
    }
    for i in &b  {
        for j in &a  {
            if i == j {
                same = true;
            }
        }
        if same != true {
            v.push(*i)
        }
        same = false;
    }
    // sortして空白区切りで出力
     v.sort();
     let mut ans: String= "".to_string();
    for i in &v  {
        ans = ans + &i.to_string();
        ans = ans + " ";
    }
}

    // println!("{}", (n + a + b).to_string() + " " + &s);
// 6 10 
// n^2 - 2