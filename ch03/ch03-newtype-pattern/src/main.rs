// Hostnameがニュータイプ
struct Hostname(String);

// 型システムによって、不正な使い方から保護される
fn connect(host: Hostname) {
    // 数値インデックスを使って、根底にあるデータにアクセスする
    println!("connected to {}", host.0);
}

fn main() {
    // 普通の文字列を作る
    let ordinary_string = String::from("localhost");
    // Hostnameを期待している関数に、普通の文字列を渡す
    let host = Hostname(ordinary_string.clone());

    /* これはエラーになる
    connect(ordinary_string);
    */

    connect(host);
}
