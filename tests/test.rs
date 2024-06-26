

#[cfg(test)]
mod tests {
    use rusty_json::base::JsonObject;
    use rusty_json::extra::{JsonEntity, JsonParser};

    #[derive(JsonEntity, Clone)]
    struct User {
        name: String,
        age: i32
    }

    #[test]
    fn test_parse_simple_json() {
        let json_input = r#"{
            "name": "John Doe",
            "age": 42,
            "email": "john.doe@example.com"
        }"#;


        let parsed_json = JsonParser::parse(json_input);
        assert!(parsed_json.is_ok());

        let json_object: JsonObject = parsed_json.unwrap().into();
        println!("{}", json_object)
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = r#"{
            "name": "John Doe"
            "age": 42,
            "email": "john.doe@example.com"
        }"#;

        let parsed_json = JsonParser::parse(invalid_json);
        assert!(parsed_json.is_err());
    }
}