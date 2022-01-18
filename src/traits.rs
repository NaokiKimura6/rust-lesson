// 基底クラスのようなもの？
trait Fruits {
    // 具体的な処理は書かずに、実際のデータ型の中で型ごとに記載
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    // ここに具体的な処理を書くとデフォルト処理となる(いわゆるオーバーライド)
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Override;
impl Summary for Override {}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}
impl Message for NewsArticle {}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let ride = Override {};
    notify(&ride);

    let article = NewsArticle {
        headline: String::from("ふなっしー登場"),
        location: String::from("千葉県"),
        author: String::from("ふなっしー"),
        content: String::from("ここから有料記事です"),
    };
    println!("{}", article.summarize());
    notify(&article);
    notifies(&article);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}
// <T: Fruits>: Genericsの中でもFruitsのtraitを持つものだけに限定する
fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}
// Summaryのtraitを実装しているデータ型であれば引数に渡せる
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 2つimplしているstructに対する処理
fn notifies(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
