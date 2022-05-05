extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자추리.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("암호는 : {}", secret_number);

    loop {
        println!("추리해보쇼.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("입력오류.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("입력형식이 올바르지 않습니다. 숫자형태로 입력하십시오.");
                continue;
            }
        };

        println!("넌 이거고름 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("작다."),
            Ordering::Greater => println!("크다."),
            Ordering::Equal => {
                println!("맞다!");
                break;
            }
        }
    }
}
