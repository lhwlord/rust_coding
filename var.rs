fn main() {
    let mut a = 1;  // i32형으로 드감
    let mut b = ""; // str로 드감

    const D: i32 = 1000;    // 규칙상 상수는 대문자. 강제는 아니지만 경고가 뜬다.

    b = "1";

    println!("{}", D);

    let a = "a";
    println!("{}", a);

    let a = 0o1_2;

    println!("{}", 5/3);

    let a: (char, i32, String) = ('가', 12, String::from("Abc"));

    let (a, b) = (i8::from(1), 2);

    let mut a = [1,2];
    a[0] = 1;

}
