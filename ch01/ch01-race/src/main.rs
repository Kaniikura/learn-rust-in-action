use std::thread; // ローカルスコープにマルチスレッドを導入
fn main() {
    let mut data = 100;
    // thread::spawn()は引数としてクロージャを受け取る
    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1000;
    });
    println!("{}", data);
}
