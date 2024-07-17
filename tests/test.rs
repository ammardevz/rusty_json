#[cfg(test)]
mod tests {
    use rusty_json::base::{JsonObject, JsonValue};
    use rusty_json::extra::{JsonEntity, JsonFormatter};
    use rusty_json_serialization::JsonEntity;

    #[derive(Debug, PartialEq, Clone, JsonEntity)] // Add Clone trait here
    struct User {
        name: String,
        age: i32,
        notes: Vec<Note>, // Change to Vec<Note> for multiple notes
    }

    #[derive(Debug, PartialEq, Clone, JsonEntity)] // Add Clone trait here
    struct Note {
        title: String,
        content: String,
    }

    #[test]
    fn test_parse_simple_json() {
        let json_input = r#"{
            "name": "John Doe",
            "age": 42,
            "notes": [
                {
                    "title": "Sample Note 1",
                    "content": "This is sample note 1."
                },
                {
                    "title": "Sample Note 2",
                    "content": "This is sample note 2."
                }
            ]
        }"#;

        // Parse JSON into User instance
        let parsed_user = User::from_raw(json_input);
        println!("Parsed user: {:?}", parsed_user);
        assert!(parsed_user.is_ok());

        let user = parsed_user.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.age, 42);
        assert_eq!(user.notes.len(), 2); // Check if there are two notes

        // Check specific note details
        assert_eq!(user.notes[0].title, "Sample Note 1");
        assert_eq!(user.notes[0].content, "This is sample note 1.");
        assert_eq!(user.notes[1].title, "Sample Note 2");
        assert_eq!(user.notes[1].content, "This is sample note 2.");

        // Serialize User instance back to JSON
        let serialized_json = user.to_json();
        let formatter = JsonFormatter::default();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("name".to_string(), JsonValue::String("John Doe".to_string()));
            obj.set("age".to_string(), JsonValue::Number(42.0));
            let mut notes_array = Vec::new();
            for note in &user.notes {
                let mut note_obj = JsonObject::new();
                note_obj.set("title".to_string(), JsonValue::String(note.title.clone()));
                note_obj.set("content".to_string(), JsonValue::String(note.content.clone()));
                notes_array.push(JsonValue::Object(note_obj));
            }
            obj.set("notes".to_string(), JsonValue::Array(notes_array.into()));
            obj
        });

        assert_eq!(serialized_json, expected_json);
        println!("Serialized JSON: {}", formatter.format(&serialized_json));
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = r#"{
            "name": "John Doe",
            "age": 42,
            "notes": {
                "title": "Invalid Note",
                "content": "This is invalid note format."
            }
        }"#;

        // Attempt to parse invalid JSON
        let parsed_user = User::from_raw(invalid_json);
        assert!(parsed_user.is_err());
        println!("Parsed user from invalid JSON: {:?}", parsed_user.err().unwrap());
    }
}
