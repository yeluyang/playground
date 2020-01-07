use std::env;

fn main() {
    for (name, value) in env::vars() {
        println!("{}={}", name, value);
    }
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
}
