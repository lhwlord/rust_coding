fn main() {
    let my_string = "Pro gram";
    let t:String = match my_string.chars().position(|c| c == ' ')    // 반환형이 Some(index)와 None이기 때문에, 이에 맞춰 match시킨다. 이거 없으면 바로 에러니깐.
    {
        Some(x) => x.to_string(),
        None => String::from("못찾음"),
    };
    // 단, chars로 들어갔으니 딱 그거만 본다. 다른 String이 있는지 확인하는건 matches로? 계속 진행할 것.


    println!("{t}");
}
