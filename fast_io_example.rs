// fast a + b, but 
use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());    // stdout, 즉 표준출력하는 핸들을 out변수에서 버퍼 쓰기용으로 설정할 것이며, 다른 개체는 못쓰게 락을 걸었음
    writeln!(out, "{}", 1234).unwrap();
}
