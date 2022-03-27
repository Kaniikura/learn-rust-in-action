#![allow(unused_variables)] // 関数内で使わない変数について警告を抑制

#[derive(Debug)]
struct File; // スタブのFile型を定義

trait Read {
    // トレイトに特定の名前を与える
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
    // トレイトのブロックは、関数実装が従うべき型シグネチャを含む。
    // 疑似型のSelfは、いつかReadを実装する型で置き換えられるプレースホルダー
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0) // 必要な型シグネチャに従う、単純なスタブの値
    }
}

fn main() {
    let f = File {};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
