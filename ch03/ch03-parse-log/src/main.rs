#[derive(Debug)]
enum Event {
    // enumは、自動的に生成されたコードを介して表示する
    Update, // Eventの3種類のバリアントを作る
    Delete, // これには「識別不能なイベント」の値も含まれる
    Unknown,
}

type Message = String; // このクレートの文脈でStringを使う便宜上の別名
                       // 1行を構文解析して、構造をもつデータに変換する関数
fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line // Vec<_>で要素の型をRustに推測させる
        .splitn(2, ' ')
        .collect(); // line.splitn()からVec<T>を返す
    if parts.len() == 1 {
        // line.split()がログを2つに分割できなければエラーを返す
        return (Event::Unknown, String::from(line)); // エラーを返す
    }
    // 後で処理しやすいようにpartsの各部を変数に割り当てておく
    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        // 既知のイベントにマッチしたら構造化したデータを返す
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)), // イベントの型を認識できなければ行全体を返す
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:L0/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
