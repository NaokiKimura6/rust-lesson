pub fn run() {
    // 所有権がmoveする場合(string型, Vector型, box pointer)(二重開放エラー回避のため)
    let s1 = String::from("hello"); // この時点では所有権はs1にある
    let s2 = s1; // この時点では所有権はs2にある
                 // println!("{}", s1); // s1に所有権はないのでエラーとなる
    println!("{}", s2);

    // 所有権がcopyされる場合(moveしない場合)(integer型、リテラル型)(参照の場合)
    let i1 = 1;
    let i2 = i1;
    println!("{}", i1);
    println!("{}", i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2); // println!("Stack address of i1 is: {:p}", &i1)と異なっているためOK

    // deep copy(同じ内容でもaddressが異なるようにできる)
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of i1 is: {:?}", &s3.as_ptr());
    println!("Heap memory address of i2 is: {:?}", &s4.as_ptr());

    // 関数の引数に指定した場合、返り値に指定した場合も所有権がなくなる
    let s5 = String::from("hello");
    take_ownership(s5);
    // take_ownership(s5); エラー
    fn take_ownership(s: String) {
        println!("Stack address of s is: {:p}", &s); // 実引数にs5を渡すと、所有権が仮引数sに行く
        println!("{}", s);
    }
    let s6 = String::from("hello");
    let s7 = take_giveback_ownership(s6);
    // println!("{}", s6); エラー
    fn take_giveback_ownership(s: String) -> String {
        s // Rustでは関数内の、文末に「;」がない最後の行がreturn値となる
    }

    // 参照で影響を受けないようにする
    let s8 = String::from("hello");
    let len = calc_len(&s8);
    println!("{}, {}", s8, len);
    fn calc_len(s: &String) -> usize {
        s.len()
    }
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);
    fn change(s: &mut String) {
        s.push_str("_world");
    }
    //mutableだと複数定義不可 immutableは複数定義可
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{}, {}, {}", s10, r1, r2);
    //mutableとimmutableは共存不可
    let mut s11 = String::from("hello");
    let r3 = &s11;
    // let r4 = mut &s11; エラー
    // println!("{}, {}, {}", s11, r3, r4);

    //mutableが有効な領域では所有権者すら権利がない
    let mut s12 = String::from("hello");
    let r5 = &mut s12;
    // println!("{}", s12); // println!("{}", r5)があるとエラー
    // println!("{}", r5);
    println!("{}", r5);
    println!("{}", s12); // 逆なら有効範囲外なのでOK

    // 借用： 所有権を移動させずに参照する権利だけを貸し出すこと
}
