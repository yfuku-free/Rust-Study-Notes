// # 課題 0

// ## 課題内容

// 1. 構造体 Person を定義してください。Person は名前と年齢を持ちます。
// 2. 列挙型 PersonType を定義してください。PersonType は Student と Teacher の 2 つのバリアントを持ちます。
// 3. PersonType ごとに Person を管理するハッシュマップを作成してください。
// 4. Person を追加、削除、リストする関数を実装してください。
// 5. ハッシュマップ内の全ての Person をイテレータを使って表示する関数を実装してください。

use std::collections::HashMap;
use std::fmt;
#[derive(Debug)]
struct Person {
    // 名前
    name: String,
    // 年齢
    age: u32,
}

#[derive(Debug, Default)]
#[derive(Eq, Hash, PartialEq)]
enum PersonType {
    // 生徒
    #[default]
    Student,
    // 先生
    Teacher
}

struct PersonManager {
    people: HashMap<PersonType, Vec<Person>>,
}
impl PersonManager {

    // 初期化
    fn init() -> Self{
        PersonManager {
            people: HashMap::new(),
        }
    }

    // 追加
    fn add_person(&mut self, person_type: PersonType, person: Person) {
        self.people.entry(person_type).or_insert(Vec::new()).push(person);
    }

    fn get_people(&self, person_type: PersonType) -> Option<&Vec<Person>> {
        self.people.get(&person_type)
    }
}

pub fn run() {
    let mut manager = PersonManager::init();

    let student = Person {
        name: String::from("Alice"),
        age: 20,
    };
    
    let teacher = Person {
        name: String::from("Bob"),
        age: 35,
    };

    let teacher2 = Person {
        name: String::from("Bob"),
        age: 35,
    };

    manager.add_person(PersonType::Student, student);
    manager.add_person(PersonType::Teacher, teacher);
    manager.add_person(PersonType::Teacher, teacher2);

    println!("Students: {:?}", manager.get_people(PersonType::Student));
    println!("Teachers: {:?}", manager.get_people(PersonType::Teacher));
}
