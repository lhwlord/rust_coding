use std::fmt::Write; // 쓰기변수에 넣어서 한번에 출력하는 용도
use std::io;
use std::io::{stdin, Read}; // 읽기용도
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_ascii_whitespace();
    // 이 다음부터 tokens에서 .next().unwrap()으로 가져오면서 적절히 사용하면 된다.
    let var: i32 = tokens.next().unwrap().parse().unwrap();

    let mut ans = 0;

    ans += var;

    // 출력은 더 간단하다. writeln!으로 변수에 꼬라박고, 마지막에 그거 출력.
    let mut output = String::new();
    write!(output, "{}", ans);

    println!("{}", output);

    // 단, 파일입력처럼 되는게 아니라 사용자와 실시간으로 입출력하려는 경우는 이게 안되니 유의할 것. 사용자의 전체 입력을 받기 때문에, 엔터 처리가 안된다.
    // 혹시 그건 전체 입력을 line으로 받아가면서 처리하면 될까...?
}
