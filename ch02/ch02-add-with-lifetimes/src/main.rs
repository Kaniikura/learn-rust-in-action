fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    // &10は10の参照、&20は20の参照
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
}
