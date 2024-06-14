// # 課題 0

// ## 課題内容

// 1. 構造体 Person を定義してください。Person は名前と年齢を持ちます。
// 2. 列挙型 PersonType を定義してください。PersonType は Student と Teacher の 2 つのバリアントを持ちます。
// 3. PersonType ごとに Person を管理するハッシュマップを作成してください。
// 4. Person を追加、削除、リストする関数を実装してください。
// 5. ハッシュマップ内の全ての Person をイテレータを使って表示する関数を実装してください。

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum PersonType {
    Student,
    Teacher
}
