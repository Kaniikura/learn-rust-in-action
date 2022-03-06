use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // スタック上の整数
    let b = Box::new(20); // ヒープ上の整数（これはボックス整数とも呼ばれる）
    let c = Rc::new(Box::new(30)); // 参照カウンタで囲んだボックス整数
    let d = Arc::new(Mutex::new(40)); // 整数をアトミックな参照カウンタで囲み、相互排他ロックで保護
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
