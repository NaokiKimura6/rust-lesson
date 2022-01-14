// import文はmod
mod vars;

fn main() {
    println!("Hello, world!");
    // vars::runでvars.tsのrun関数を実行
    vars::run();
    vars::sub_a::func_a();
    vars::sub_b::func_b();
}
