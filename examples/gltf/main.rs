use serde_json::json;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
    pub inner: Option<usize>,
}

fn main() {
    let ex = Data { inner: None };
    let j = json!(ex);
    println!("{}", j);
}
