use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    println!("{now:?}");
    std::thread::sleep(std::time::Duration::new(2, 0));

    let now = SystemTime::now();
    println!("{now:?}");
}
