use std::io;
use std::io::{stdin, Read}; // 읽기용도
use io::Write;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();


    // 출력 설정용.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.split_ascii_whitespace();
        // 이 다음부터 tokens에서 .next().unwrap()으로 가져오면서 적절히 사용하면 된다.

        let var1: i32 = tokens.next().unwrap().parse().unwrap();

        let var2: i32 = tokens.next().unwrap().parse().unwrap();

        // 출력은 더 간단하다. writeln!을 stdout 핸들에 꼬라박으면 된다.
        writeln!(out, "{}", var1 + var2).unwrap();

    }
    // 실시간 처리가 필요하다면, read_line으로 직접 받으며, 출력버퍼를 설정해서 거기에 쓰는 식으로 구현할 수 있다.
    // 메모리는 통상적인 크기(13메가)로 잡아먹지만, 속도는 파일 입출력에 비해 좀 느리다. 그래도 백준을 충분히 통과하는 212ms가 나온다.
}
