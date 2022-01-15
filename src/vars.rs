pub fn run() {
    let x = 5;
    println!("{}", x);
    // x = 6; // error
    let mut y = 5;
    println!("{}", y);
    y = 6;
    println!("{}", y);

    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS); // 64
    println!("Memory address of const is: {}", &MAX_POINTS); // 格納番地を取得
    println!("Memory address of const is: {:p}", &MAX_POINTS); // 格納番地を取得(ポインター(16進数))

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);
    let a = 5;
    println!("Stack address of a is: {:p}", &a);
    let a = a + 1;
    println!("Stack address of a is: {:p}", &a);
    let a = a * 2;
    println!("Stack address of a is: {:p}", &a);
    println!("{}", a);
    {
        let a = 0;
        println!("{}", a);
    }
    println!("{}", a);

    // tuple
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("Stack address of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut t01, ref mut t02), _) = t2;
    *t01 = 4;
    *t02 = 5;
    println!("{:?}", t2); // primitive出ない場合はブラケットの中に:?を記載

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?}, {:?}", a1, a2);

    let s1 = "helloこんにちは挨拶"; // UTF-8 // 26bytes
    let s2 = "hello";
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Static memory address of s1 is: {:?}", &s1.as_ptr());
    println!("Static memory address of s2 is: {:?}", &s2.as_ptr());
    println!("Len of s1 is: {}", &s1.len());
    println!("Len of s2 is: {}", &s2.len());

    let mut s3 = String::from("hello"); // 24bytes
    let mut s4 = String::from("helloworld");
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of s3 is: {:?}", &s3.as_ptr());
    println!("Heap memory address of s4 is: {:?}", &s4.as_ptr());
    println!("Len of s3 is: {}", &s3.len());
    println!("Len of s4 is: {}", &s4.len());
    println!("Capacity of s3 is: {}", &s3.capacity());
    println!("Capacity of s4 is: {}", &s4.capacity());
    s3.push_str("_new3");
    s4.push_str("_new4");
    println!("{}, {}", s3, s4);
}
