#[cfg(test)]
mod test {
    use std::error::Error;
    use rusty_json::extra::JsonParser;
    use rusty_json::base::JsonValue;
    use rusty_json::{is_null, json};

    #[test]
    fn simple_cast() -> Result<(), Box<dyn Error>> {
        let name_json = json!("Ammar Dev");
        let age_json = json!(27);
        let hobbies_json = json!(["coding", "IDK"]);

        let name = name_json.parse::<String>()?;
        let age = age_json.parse::<i32>()?;
        let hobbies = hobbies_json.parse::<Vec<String>>()?;

        println!("name: {}, age: {}, hobbies: {:?}", name, age, hobbies);
        Ok(())
    }

    #[test]
    fn cast_boolean_and_null() -> Result<(), Box<dyn Error>> {
        let true_json = json!(true);
        let false_json = json!(false);
        let null_json = json!(null);

        let true_value = true_json.parse::<bool>()?;
        let false_value = false_json.parse::<bool>()?;

        assert_eq!(true_value, true);
        assert_eq!(false_value, false);
        assert!(is_null!(null_json));

        Ok(())
    }

    #[test]
    fn cast_floating_point() -> Result<(), Box<dyn Error>> {
        let float_json = json!(std::f64::consts::PI);
        let float_value = float_json.parse::<f64>()?;

        assert!((float_value - std::f64::consts::PI).abs() < f64::EPSILON);

        Ok(())
    }

    #[test]
    fn cast_complex_object() -> Result<(), Box<dyn Error>> {
        let complex_json = json!({
            name: "Ammar Dev",
            age: 27,
            hobbies: ["coding", "IDK"],
            is_developer: true,
            address: {
                street: "123 Main St",
                city: "Anytown"
            }
        });

        let name = complex_json["name"].parse::<String>()?;
        let age = complex_json["age"].parse::<i32>()?;
        let hobbies = complex_json["hobbies"].clone().parse::<Vec<String>>()?;
        let is_developer = complex_json["is_developer"].parse::<bool>()?;
        let street = complex_json["address"]["street"].parse::<String>()?;
        let city = complex_json["address"]["city"].parse::<String>()?;

        assert_eq!(name, "Ammar Dev");
        assert_eq!(age, 27);
        assert_eq!(hobbies, vec!["coding".to_string(), "IDK".to_string()]);
        assert_eq!(is_developer, true);
        assert_eq!(street, "123 Main St");
        assert_eq!(city, "Anytown");

        Ok(())
    }

    #[test]
    fn parse_and_cast_json_string() -> Result<(), Box<dyn Error>> {
        let json_str = r#"
        {
            "name": "Ammar Dev",
            "age": 27,
            "hobbies": ["coding", "IDK"],
            "is_developer": true
        }
        "#;

        let parsed_json = JsonParser::parse(json_str)?;

        match parsed_json {
            JsonValue::Object(ref obj) => {
                let name = obj["name"].clone().parse::<String>()?;
                let age = obj["age"].clone().parse::<i32>()?;
                let hobbies = obj["hobbies"].clone().parse::<Vec<String>>()?;
                let is_developer = obj["is_developer"].clone().parse::<bool>()?;

                assert_eq!(name, "Ammar Dev");
                assert_eq!(age, 27);
                assert_eq!(hobbies, vec!["coding".to_string(), "IDK".to_string()]);
                assert_eq!(is_developer, true);
            },
            _ => panic!("Parsed JSON is not an object"),
        }

        Ok(())
    }

    #[test]
    fn cast_nested_arrays_and_objects() -> Result<(), Box<dyn Error>> {
        let nested_json = json!({
            nested_array: [[1, 2], [3, 4]],
            nested_object: {
                inner_object: {
                    key: "value"
                }
            }
        });

        let nested_array = nested_json["nested_array"].clone().parse::<Vec<Vec<i32>>>()?;
        let inner_key = nested_json["nested_object"]["inner_object"]["key"].parse::<String>()?;

        assert_eq!(nested_array[0][0], 1);
        assert_eq!(nested_array[1][1], 4);
        assert_eq!(inner_key, "value");

        Ok(())
    }

    #[test]
    fn cast_to_various_types() -> Result<(), Box<dyn Error>> {
        let int_json = json!(42);
        let float_json = json!(3.14);
        let bool_json = json!(true);
        let string_json = json!("example");

        let int_value = int_json.parse::<i32>()?;
        let float_value = float_json.parse::<f64>()?;
        let bool_value = bool_json.parse::<bool>()?;
        let string_value = string_json.parse::<String>()?;

        assert_eq!(int_value, 42);
        assert!((float_value - 3.14).abs() < f64::EPSILON);
        assert_eq!(bool_value, true);
        assert_eq!(string_value, "example");

        Ok(())
    }

    #[test]
    fn cast_json_with_numbers() -> Result<(), Box<dyn Error>> {
        let json_data = json!({
            integer: 42,
            floating: 3.14
        });

        let integer = json_data["integer"].parse::<i32>()?;
        let floating = json_data["floating"].parse::<f64>()?;

        assert_eq!(integer, 42);
        assert!((floating - 3.14).abs() < f64::EPSILON);

        Ok(())
    }

    #[test]
    fn cast_json_with_booleans_and_null() -> Result<(), Box<dyn Error>> {
        let json_data = json!({
            true_value: true,
            false_value: false,
            null_value: null
        });

        let true_value = json_data["true_value"].parse::<bool>()?;
        let false_value = json_data["false_value"].parse::<bool>()?;
        let null_value: JsonValue = json_data["null_value"].parse()?;

        assert_eq!(true_value, true);
        assert_eq!(false_value, false);
        assert!(is_null!(null_value));

        Ok(())
    }

    #[test]
    fn cast_json_with_arrays() -> Result<(), Box<dyn Error>> {
        let json_data = json!({
            numbers: [1, 2, 3, 4, 5],
            strings: ["one", "two", "three"],
            mixed: [1, "two", true, null]
        });

        let numbers = json_data["numbers"].clone().parse::<Vec<i32>>()?;
        let strings = json_data["strings"].clone().parse::<Vec<String>>()?;
        let mixed = json_data["mixed"].clone().parse::<Vec<JsonValue>>()?;

        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(strings, vec!["one".to_string(), "two".to_string(), "three".to_string()]);
        assert_eq!(mixed[0], JsonValue::Number(1.into()));
        assert_eq!(mixed[1], JsonValue::String("two".to_string()));
        assert_eq!(mixed[2], JsonValue::Boolean(true));
        assert!(is_null!(mixed[3]));

        Ok(())
    }
}