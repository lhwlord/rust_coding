use std::io;

fn main() {
    println!("숫자를 추리하시오 :");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("읽기 실패");

    println!("당신의 추리 : {}", guess);
    
}
