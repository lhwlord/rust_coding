use std::io;
use std::io::{stdin, Read}; // 읽기용도
use std::fmt::Write;
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_ascii_whitespace();

    let t: i32 = tokens.next().unwrap().parse().unwrap();


    // 출력 설정용.
    let mut out = String::new();

    for _ in 0..t {
        
        // 이 다음부터 tokens에서 .next().unwrap()으로 가져오면서 적절히 사용하면 된다.

        let var1: i32 = tokens.next().unwrap().parse().unwrap();

        let var2: i32 = tokens.next().unwrap().parse().unwrap();

        // 파일출력인 경우 io::Write 대신 std::fmt::Write가 필요하다.
        writeln!(out, "{}", var1 + var2).unwrap();
    }
    
    println!("{}", out);
    
    // 파일 단위로 받는다면, read_line 대신 read_to_string을, 출력버퍼 대신 변수 하나에 담고 한번에 보내는 식으로 동작하면 3배가량 빠르게 할 수 있다.
    // 다만 메모리도 장난아니게 먹는다. 백준기준 30메가.
}
