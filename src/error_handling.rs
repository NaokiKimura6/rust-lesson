pub fn run() {
    let res1 = division_option(5.0, 0.0);
    // Option型エラーハンドリング
    match res1 {
        // 小数点以下3桁まで表示
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Not allowed !!"),
    }
    let res2 = division_result(5.0, 0.0);
    // Result型エラーハンドリング
    match res2 {
        // 小数点以下3桁まで表示
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("{}", e),
    }
    let res3 = sum(&[0, 1, 2]);
    let res4 = sum(&[0, 1]);
    match res3 {
        Some(x) => println!("Sum is: {}", x),
        None => println!("Out of index !!"),
    }
    match res4 {
        Some(x) => println!("Sum is: {}", x),
        None => println!("Out of index !!"),
    }
}
// Option型エラーハンドリング
fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
// Result型エラーハンドリング
fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed !!"))
    } else {
        Ok(x / y)
    }
}

// 配列の存在しないindexにアクセスした場合
fn sum(a: &[i32]) -> Option<i32> {
    // 「?」によりOutOfIndexを回避できる。その場合Noneを返す
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
