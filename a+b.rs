use std::io::{prelude::*};

fn main()
{
    // 벡터 이용하는 방법
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    let split_string: Vec<&str> = a.split(' ').collect();    // 벡터에 공백기준으로 자르기
    let a:i32 = split_string[0].trim().parse().expect("에바");
    let b:i32 = split_string[1].trim().parse().expect("err");
    println!("{}", a+b);

    // 요상한거 쓰는 방법 : 근데 이건 버퍼별로 받은거라서 자릿수 문제가 생긴다. 백준에서야 그냥 넘겼지만... 이건 범용성이 없다.
    // let mut buf = [0;3];
    // std::io::stdin().read(&mut buf).unwrap();
    // println!("{}", buf[0] + buf[2] - b'0'*2);
}