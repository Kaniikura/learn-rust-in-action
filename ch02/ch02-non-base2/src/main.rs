fn main() {
    let three = 0b11; // 接頭辞0bは、binary（2進数）を示す
    let thirty = 0o36; // 接頭辞0oは、octal（8進数）を示す
    let three_hundred = 0x12C; // 接頭辞0xは、hexadecimal（16進数）を示す

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
