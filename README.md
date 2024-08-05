Here's a revised version of your README that includes a note about the crate no longer receiving updates and suggests using `serde_json` instead:

---

# Rusty Json

A lightweight and straightforward JSON library for Rust, providing basic JSON functionality, serialization, and manipulation.

> **Note:** This crate is no longer being actively maintained due to various issues and limited development skills. For a more robust and widely used JSON library, consider using `serde_json`. While `Rusty Json` is designed to be simple and lightweight, it may not be as performant or feature-rich as `serde_json`.

## Usage

### Base 

- **Json Object**

```rust
let mut json_obj = JsonObject::new();
json_obj.set("Name", "Ammar Dev");
json_obj.set("msg", "Github");
json_obj.set("number", 234);

println!("{}", json_obj);
```

```json
{"msg": "Github", "Name": "Ammar Dev", "number": 234}
```

- **Json Array**

```rust
let mut json_arr = JsonArray::new();
json_arr.push("Value 1");
json_arr.push(324);
json_arr.push(vec![1, 2]);

println!("{}", json_arr);
```

```json
["Value 1", 324, [1, 2]]
```

OR

```rust
let mut json_arr: JsonArray = vec![1, 2].into();
json_arr.push("Value 1");
json_arr.push("Value 2");

println!("{}", json_arr);
```

```json
[1, 2, "Value 1", "Value 2"]
```

- **Json Value**

```rust
fn main() -> Result<(), Box<dyn Error>> {
    // Creating JSON values of different types
    let string_value = JsonValue::String("Hello".to_string());
    let number_value_1 = JsonValue::Number(3453f64);
    let number_value_2 = JsonValue::Number(4234001f64);
    let bool_value = JsonValue::Boolean(false);
    let null_value = JsonValue::Null;
    let object_value = JsonValue::Object(JsonObject::new());
    let array_value = JsonValue::Array(JsonArray::new());

    // Parsing the JSON values into Rust types
    let parsed_string: String = string_value.parse()?;
    let parsed_number_1: f64 = number_value_1.parse()?;
    let parsed_number_2: i32 = number_value_2.parse()?;
    let parsed_bool: bool = bool_value.parse()?;

    // Printing the parsed values
    println!("Parsed String: {}", parsed_string);
    println!("Parsed Number 1: {}", parsed_number_1);
    println!("Parsed Number 2: {}", parsed_number_2);
    println!("Parsed Bool: {}", parsed_bool);

    // Indicating that the main function completed successfully
    Ok(())
}
```

## Extra

- **Parser**

```rust
let raw_json = r#"
    {
        "name": "Ammar Dev",
        "age": 99,
        "email": "AmmarDev@example.com",
        "isEmployed": true
    }
"#;

let json_val = JsonParser::parse(raw_json).unwrap(); // Parsed into JsonValue
let mut json_obj = JsonObject::from(json_val); // Cast into Json Object
json_obj.set("isEmployed", false);
println!("{}", json_obj);
```

```json
{"name": "Ammar Dev", "age": 99, "email": "AmmarDev@example.com", "isEmployed": false}
```

- **Formatter**

```rust
let mut json_obj = JsonObject::new();
json_obj.set("name", "Ammar Dev");
json_obj.set("age", 99);

let mut json_arr = JsonArray::from(vec![1, 2]);
json_arr.push("Hi");
json_arr.push("Hello");

json_obj.set("list_example", json_arr);

let formatter = JsonFormatter::default();
println!("{}", formatter.format(json_obj));
```

```json
{
  "name": "Ammar Dev",
  "age": 99,
  "list_example": [
    1,
    2,
    "Hi",
    "Hello"
  ]
}
```

- **Macros**

```rust
let json_obj = json!({
    name: "Ammar Dev",
    Age: 99
}); // will produce JsonObject

let json_arr = json!([
    1,
    2,
    "String 1",
    "String 2"
]); // will produce JsonArray

let json_val = json!("Hello"); // will produce JsonValue
```

## Implementing

- **Json Entity (manually)**

```rust
#[derive(Clone)]
struct User {
    name: String,
    age: i32,
}

impl JsonEntity for User {
    fn to_json(&self) -> JsonValue {
        let mut obj = JsonObject::new();
        obj.set("name", &self.name);
        obj.set("age", &self.age);
        JsonValue::Object(obj)
    }

    fn from_json(raw: &str) -> Result<Self, ConversationError> {
        let parsed = JsonParser::parse(raw)?;
        if let JsonValue::Object(obj) = parsed {
            let name = obj.get("name").ok_or_else(|| ConversationError::GenericError("Missing 'name'".to_string()))?
                .parse::<String>()?;
            let age = obj.get("age").ok_or_else(|| ConversationError::GenericError("Missing 'age'".to_string()))?
                .parse::<i32>()?;
            Ok(User { name, age })
        } else {
            Err(ConversationError::GenericError("Expected JSON object".to_string()))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let user = User {
        name: "Ammar Dev".to_string(),
        age: 22,
    };

    let json = user.to_json();
    println!("{}", json);

    let json_str = json.to_string(); // Assuming to_string() is implemented
    let parsed_user = User::from_json(&json_str)?;

    println!("{}", parsed_user.to_json());

    Ok(())
}
```

```json
{"name": "Ammar Dev", "age": 22}
```

- **Json Entity (Auto) [`Requires: serializing feature`]**

```rust
#[derive(JsonEntity, Clone, Debug)] // Derive JsonEntity for automatic serialization and deserialization
struct User {
    name: String,
    age: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of User
    let user = User {
        name: "Ammar Dev".to_string(),
        age: 22,
    };

    // Serialize User to JSON and print it
    println!("Serialized JSON:\n{}", user.to_json());

    // Example JSON string for deserialization
    let json_str = r#"{"name": "Ammar Dev", "age": 22}"#;

    // Deserialize JSON back to User
    let deserialized_user: User = User::from_json(json_str)?;

    // Print deserialized User
    println!("Deserialized User: {:?}", deserialized_user);

    Ok(())
}
```

```json
{"name": "Ammar Dev", "age": 22}
```

## Copyrights

All rights reserved for Ammar Dev <br>
This project is licensed with [MIT LICENSE](https://github.com/ammardevz/rusty_json/blob/master/LICENSE)

---

Feel free to adjust the details according to your needs!
