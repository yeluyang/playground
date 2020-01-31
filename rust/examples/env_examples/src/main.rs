use std::env;

fn main() {
    println!(
        "{} - {}, by {}, at {}",
        env::var("CARGO_PKG_NAME").unwrap(),
        env::var("CARGO_PKG_VERSION").unwrap(),
        env::var("CARGO_PKG_AUTHORS").unwrap(),
        env::var("CARGO_MANIFEST_DIR").unwrap(),
    );
    assert_eq!(
        format!(
            "{}.{}.{}",
            env::var("CARGO_PKG_VERSION_MAJOR").unwrap(),
            env::var("CARGO_PKG_VERSION_MINOR").unwrap(),
            env::var("CARGO_PKG_VERSION_PATCH").unwrap()
        ),
        env::var("CARGO_PKG_VERSION").unwrap(),
    );
    println!(
        "current dir: {}",
        env::current_dir().unwrap().to_string_lossy()
    );
    println!(
        "current exe: {}",
        env::current_exe().unwrap().to_string_lossy()
    );
    println!("tmp dir: {}", env::temp_dir().to_string_lossy());
    // [DEPRECATED] output home dir of user
    // println!(
    //     "home dir: {}",
    //     env::home_dir().unwrap().to_string_lossy()
    // );
    for (name, value) in env::vars() {
        println!("{}={}", name, value);
    }
}
