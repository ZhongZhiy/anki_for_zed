use serde::Serialize;
use serde_json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")] // 自动把 deck_name 变成 deckName
struct Note {
    deck_name: String,
    model_name: String,
    fields: Fields,
}

#[derive(Serialize)]
struct Fields {
    #[serde(rename = "Front")] // 强制改名，匹配 Anki 字段
    front: String,
    #[serde(rename = "Back")]
    back: String,
}

// 这是最外层的请求壳子
#[derive(Serialize)]
struct AnkiRequest {
    action: String,
    version: u8,
    params: NoteContainer,
}

#[derive(Serialize)]
struct NoteContainer {
    note: Note,
}

fn main() {
    // 1. 创建数据
    let my_note = Note {
        deck_name: "Default".to_string(),
        model_name: "Basic".to_string(),
        fields: Fields {
            front: "Hello".to_string(),
            back: "世界".to_string(),
        },
    };

    let request = AnkiRequest {
        action: "addNote".to_string(),
        version: 6,
        params: NoteContainer { note: my_note },
    };

    // 2. 序列化：把结构体变成 JSON 字符串
    // 这是你真正要发给 Anki-Connect 的东西
    let json_string = serde_json::to_string(&request).unwrap();

    println!("准备发送给 Anki 的内容：\n{:?}", json_string);
}
