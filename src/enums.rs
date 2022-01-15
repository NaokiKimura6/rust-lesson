enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let windows = OS::Windows(1985, String::from("MicroSoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(mac);
    let linux = OS::Linux(1991, String::from("Linus"));
    print_os_info(linux);
}
fn print_os_info(os: OS) {
    // Switch文と本質的に同じ
    match os {
        OS::Windows(year, who) => {
            println!("Windows : First release in {} by {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Mac : First release in {} by {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linux : First release in {} by {}", year, who);
        }
    }
}
