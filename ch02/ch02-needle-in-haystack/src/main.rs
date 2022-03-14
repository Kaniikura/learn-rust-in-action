fn main() {
    let needle = 0o64;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &haystack {
        // haystack配列に含まれる各要素への参照を反復処理する
        if *item == needle {
            // *itemという構文は、itemの参照先を返す
            println!("{}", item);
        }
    }
}
