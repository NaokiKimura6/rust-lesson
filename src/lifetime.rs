pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res12 = get_longest(&st1, &st2);
    println!("{}", res12);
    let st3 = String::from("x");
    {
        let st4 = String::from("y");
        let res34 = get_longest(&st3, &st4);
        println!("{}", res34);
    }
}
// 返り値のreferenceは引数のLifetimeのうち短い方を指し示す
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 関数を抜ける際に実態がdropされ、referenceだけ残るのでエラーとなる
// fn dummy<'a>() -> &'a str {
//     &String::from("demo")
// }
