fn main() {
    let default_value = serde_json::Value::default();
    if default_value.is_null() {
        println!("{default_value:?}")
    }
    println!("{default_value:?}")
}
