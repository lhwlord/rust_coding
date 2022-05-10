fn main()
{
    let a = String::from("abc def");
    let b:i32 = match a.find("c d") // match를 알고 나니 이제 메소드 종류만 알면 되는 느낌이 됐다. 이건 문자열 단위로 찾으니 더 편하다. 없으면 없는대로 표현도 가능.
    {
        Some(x) => x as i32,
        None => -1,
    };
    println!("{b}");
}