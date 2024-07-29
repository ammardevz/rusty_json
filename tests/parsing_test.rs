#[cfg(test)]
mod test {
    use std::error::Error;

    use rusty_json::base::JsonValue;
    use rusty_json::extra::JsonParser;

    #[test]
    fn parse_large_json() -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string("tests/flats.json")?;
        let parsed = JsonParser::parse(&content);
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn parse_large_complex_json() -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string("tests/complex_flats.json")?;
        let parsed = JsonParser::parse(&content);
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn parse_json_with_unicode_characters() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "こんにちは": "Hello in Japanese",
            "你好": "Hello in Chinese"
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                // Check if specific keys with Unicode characters exist
                assert!(obj.contains_key("こんにちは"));
                assert!(obj.contains_key("你好"));

                // Check if the values are as expected
                assert_eq!(obj["こんにちは"], JsonValue::String("Hello in Japanese".to_string()));
                assert_eq!(obj["你好"], JsonValue::String("Hello in Chinese".to_string()));
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn parse_json_with_escape_characters() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "escaped": "Hello\nWorld",
            "tabbed": "Hello\tWorld",
            "quoted": "He said, \"Hello World\"",
            "backslash": "C:\\\\Program Files\\\\",
            "slash": "http://example.com/path/to/resource",
            "carriage_return": "Hello\rWorld",
            "mixed": "Line1\nLine2\r\nLine3\tTabbed"
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                // Check if specific keys exist
                assert!(obj.contains_key("escaped"));
                assert!(obj.contains_key("tabbed"));
                assert!(obj.contains_key("quoted"));
                assert!(obj.contains_key("backslash"));
                assert!(obj.contains_key("slash"));
                assert!(obj.contains_key("carriage_return"));
                assert!(obj.contains_key("mixed"));

                // Check if the values are as expected
                assert_eq!(obj["escaped"], JsonValue::String("Hello\nWorld".to_string()));
                assert_eq!(obj["tabbed"], JsonValue::String("Hello\tWorld".to_string()));
                assert_eq!(obj["quoted"], JsonValue::String("He said, \"Hello World\"".to_string()));
                assert_eq!(obj["backslash"], JsonValue::String("C:\\\\Program Files\\\\".to_string()));
                assert_eq!(obj["slash"], JsonValue::String("http://example.com/path/to/resource".to_string()));
                assert_eq!(obj["carriage_return"], JsonValue::String("Hello\rWorld".to_string()));
                assert_eq!(obj["mixed"], JsonValue::String("Line1\nLine2\r\nLine3\tTabbed".to_string()));
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn parse_empty_json_object() -> Result<(), Box<dyn Error>> {
        let content = r#"{}"#;
        let parsed = JsonParser::parse(content)?;
        match parsed {
            JsonValue::Object(ref obj) => {
                assert!(obj.is_empty());
            }
            _ => panic!("Parsed JSON is not an object"),
        }
        Ok(())
    }

    #[test]
    fn parse_empty_json_array() -> Result<(), Box<dyn Error>> {
        let content = r#"[]"#;
        let parsed = JsonParser::parse(content)?;
        match parsed {
            JsonValue::Array(ref arr) => {
                assert!(arr.is_empty());
            }
            _ => panic!("Parsed JSON is not an array"),
        }
        Ok(())
    }

    #[test]
    fn parse_nested_json_structure() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "level1": {
                "level2": {
                    "level3": "Nested value"
                }
            }
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                assert_eq!(obj["level1"]["level2"]["level3"], JsonValue::String("Nested value".to_string()));
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn parse_json_with_numbers() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "integer": 42,
            "floating": 3.14
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                assert_eq!(obj["integer"], JsonValue::Number(42.into()));
                assert_eq!(obj["floating"], JsonValue::Number(3.14.into()));
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn parse_json_with_booleans_and_null() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "true_value": true,
            "false_value": false,
            "null_value": null
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                assert_eq!(obj["true_value"], JsonValue::Boolean(true));
                assert_eq!(obj["false_value"], JsonValue::Boolean(false));
                assert_eq!(obj["null_value"], JsonValue::Null);
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn parse_json_with_arrays() -> Result<(), Box<dyn Error>> {
        let content = r#"
        {
            "numbers": [1, 2, 3, 4, 5],
            "strings": ["one", "two", "three"],
            "mixed": [1, "two", true, null]
        }
        "#;

        let parsed = JsonParser::parse(content)?;

        match parsed {
            JsonValue::Object(ref obj) => {
                assert_eq!(obj["numbers"][0], JsonValue::Number(1.into()));
                assert_eq!(obj["numbers"][4], JsonValue::Number(5.into()));
                assert_eq!(obj["strings"][1], JsonValue::String("two".to_string()));
                assert_eq!(obj["mixed"][0], JsonValue::Number(1.into()));
                assert_eq!(obj["mixed"][1], JsonValue::String("two".to_string()));
                assert_eq!(obj["mixed"][2], JsonValue::Boolean(true));
                assert_eq!(obj["mixed"][3], JsonValue::Null);
            }
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }
}
