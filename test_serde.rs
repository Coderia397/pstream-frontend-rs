use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Test {
    a: Option<String>,
}

fn main() {
    let json = "{}";
    let t: Result<Test, _> = serde_json::from_str(json);
    println!("{:?}", t);
}
