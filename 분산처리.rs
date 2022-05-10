use std::io::stdin;
use std::fmt::Write;

fn main()
{
    let mut t = String::new();
    stdin().read_line(&mut t).unwrap();

    let t = t.trim().parse().unwrap();

    let mut out = String::new();

    for _ in 0..t {

        let mut temp = String::new();
        stdin().read_line(& mut temp).unwrap();
        let mut temp = temp.split_ascii_whitespace();   // 넌 이제 토큰이다.

        let a:u32 = temp.next().unwrap().parse().unwrap();
        let b:u32 = temp.next().unwrap().parse().unwrap();

        let mut c = 1;

        for _ in 0..b{
            c = c * a % 10;
            if c == 0 {c += 10;}
        }

        writeln!(out, "{}", c).unwrap();
    }

    println!("{}", out);
}