#![allow(unused_variables)] // コンパイラの警告を緩和

type File = String; // 型の別名を作る

fn open(f: &mut File) -> bool {
    true // この関数は常に成功するものとする
}

fn close(f: &mut File) -> bool {
    true // この関数は常に成功するものとする
}

#[allow(dead_code)] // 一度も使わない関数へのコンパイラの警告も緩和
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // 到達したらプログラムをクラッシュさせる「未実装」マクロ
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}
