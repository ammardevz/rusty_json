#[cfg(test)]
mod tests {
    use rusty_json::base::{JsonObject, JsonValue};
    use rusty_json::extra::{JsonEntity, JsonFormatter};
    use rusty_json_serialization::JsonEntity;

    #[derive(Debug, PartialEq, JsonEntity)]
    struct User {
        name: String,
        age: i32,
        note: Note
    }

    #[derive(Debug, PartialEq, JsonEntity)]
    struct Note {
        title: String,
        content: String
    }

    #[test]
    fn test_parse_simple_json() {
        let json_input = r#"{
            "name": "John Doe",
            "age": 42,
            "note": {
                "title": "Sample Note",
                "content": "This is a sample note."
            }
        }"#;

        // Parse JSON into User instance
        let parsed_user = User::from_raw(json_input);
        println!("Parsed user: {:?}", parsed_user);
        assert!(parsed_user.is_ok());

        let user = parsed_user.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.age, 42);
        assert_eq!(user.note.title, "Sample Note");
        assert_eq!(user.note.content, "This is a sample note.");

        // Serialize User instance back to JSON
        let serialized_json = user.to_json();
        let formatter = JsonFormatter::default();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("name".to_string(), JsonValue::String("John Doe".to_string()));
            obj.set("age".to_string(), JsonValue::Number(42.0));
            obj.set("note".to_string(), JsonValue::Object({
                let mut note_obj = JsonObject::new();
                note_obj.set("title".to_string(), JsonValue::String("Sample Note".to_string()));
                note_obj.set("content".to_string(), JsonValue::String("This is a sample note.".to_string()));
                note_obj
            }));
            obj
        });

        assert_eq!(serialized_json, expected_json);
        println!("Serialized JSON: {}", formatter.format(&serialized_json));
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = r#"{
            "name": "John Doe"
            "age": 42
        }"#;

        // Attempt to parse invalid JSON
        let parsed_user = User::from_raw(invalid_json);
        assert!(parsed_user.is_err());
        println!("Parsed user from invalid JSON: {:?}", parsed_user.err().unwrap());
    }
}
