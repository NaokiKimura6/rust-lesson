#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // インスタンス生成
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // 所有権移動を防ぐためreference化
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    // インスタンス化
    let mut user1 = User {
        username: String::from("someusername1"),
        email: String::from("someone@examole.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("someone1@examole.com");
    println!("{:#?}", user1);

    let user2 = build_user(
        String::from("someusername2"),
        String::from("someone2@examole.com"),
        2,
    );
    println!("{:#?}", user2);
    let rect = Rectangle::create(20, 30);
    println!("{:#?}", rect);
    rect.area();
}
fn build_user(username: String, email: String, sign_in_count: u64) -> User {
    User {
        username,
        email,
        sign_in_count,
        active: true,
    }
}
