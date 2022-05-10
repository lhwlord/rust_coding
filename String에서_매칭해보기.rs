fn main()
{
    let a = String::from("abcd ef");
    let b = a.matches("ef");    // 매치결과에 대한 Match를 반환하는데, 얘는 많은 정보를 담고 있다. 입맛대로 사용 가능(벡터, next 등등)
    let c:i32;
    if b.clone().count() == 0   // 클론 안하면 저 내부 메소드에서 b가 불려나가서 이동된다.
    {
        c = 0;
    }
    else {c = b.count() as i32;}

    // 이거 말고 a.contains("abc") 이건 안에 있는지만을 확인해준다고 한다. String 관련 메소드를 잘 알아야 할듯.

    println!("{c}");
}