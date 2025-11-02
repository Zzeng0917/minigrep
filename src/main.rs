use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);//终结进程，因为错误而退出
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

