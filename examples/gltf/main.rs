#[derive(serde::Serialize, serde::Deserialize)]
enum Block {
    #[serde(rename = "POSITION")]
    Pos(i32),
    Vec(i32),
}

fn main() {
    let a = Block::Pos(1);
    let b = serde_json::to_string(&a).unwrap();
    println!("{b}");
}
