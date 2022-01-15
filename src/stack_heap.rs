enum List {
    // 無限サイクルを解決するためbox pointer化
    Node(i32, Box<List>),
    Nil, // 0byte
}

pub fn run() {
    let _a1: [u8; 1000000] = [1; 1000000];

    // Vector型
    let mut v1 = vec![1, 2, 3, 4]; // 24bytes メモリ構造はstring型と同じ
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1 is: {:?}", &v1.as_ptr());
    println!("Len of v1 is: {}", &v1.len());
    println!("Capacity of v1 is: {}", &v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3); // 空になる！！
                          //box pointer
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1);
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{}, {}", b1.0, b1.1);
    println!("Stack address of box pointer is: {:p}", &b1);
    println!("Heap memory address of box pointer is: {:p}", b1); // println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr())と同じ

    // List構造
}
