#[serde_ext::extend_serde]
#[derive(Debug, serde::Deserialize)]
pub struct Foo {
    #[serde_ext(default(literal = 23))]
    a: i32,
    #[serde_ext(default(inline = r#"|| String::from("Hello") "#))]
    b: String,
    #[serde(deserialize_with = "serde_ext::de::parsable")]
    url: url::Url,
    #[serde(with = "serde_ext::base64")]
    base64_bytes: Vec<u8>,
    #[serde(deserialize_with = "serde_ext::de::non_empty_string")]
    non_empty_string: Option<String>
}

fn main() {

    println!(
        "{:?}",
        serde_json::from_str::<Foo>(r#"{ 
            "a": 22, 
            "bar": "x", 
            "url": "http://google.com", 
            "base64_bytes": "aGVsbG8gd29ybGQ=",
            "non_empty_string": "a"
        }"#)
    );
}
