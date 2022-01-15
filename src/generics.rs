// 構造体
struct Point<T> {
    x: T,
    y: T,
}
struct PointAnother<T, U> {
    x: T,
    y: U,
}
// mix
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largets_i32(number_list));
    // ''で囲むとchar型になる
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", larget(char_list));
    println!("{}", larget(number_list));
    // 実態(インスタンス化 オブジェクト指向っぽい)
    let p1 = Point { x: 1, y: 2 };
    let p2 = PointAnother { x: 1.0, y: 2 };
    let p3 = PointAnother { x: 5, y: 0.4 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{}, {}", p5.x, p5.y);
}
fn largets_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
// PartialOrd + Copy: 比較演算が可能となる型(全順序となる型)に絞る
fn larget<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
