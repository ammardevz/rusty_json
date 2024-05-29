
# Rusty Json 

a Simple json library for rust, it has various ways to use and the library/crate its totally written in rust its still under development 

#  Usage

## Base 

- Json Object

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

- Json Array

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

- Json Value

```rust
let json_string_value = JsonValue::String("Hello".to_string());
let json_number_value_1 = JsonValue::Number(JsonNumber::I32(3453));
let json_number_value_2 = JsonValue::Number(4234001.into());
let json_value_bool = JsonValue::Boolean(false);
let json_null_value = JsonValue::Null;
let json_value_obj = JsonValue::Object(JsonObject::new());
let json_value_arr = JsonValue::Array(JsonArray::new());
```

## Extra

- Parser

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

- Prettier

```rust
let mut json_obj = JsonObject::new();
json_obj.set("name", "Ammar Dev");
json_obj.set("age", 99);

let mut json_arr = JsonArray::from(vec![1, 2]);
json_arr.push("Hi");
json_arr.push("Hello");

json_obj.set("list_example", json_arr);

extra::prettier::set_indent(2); // indent is 2 by default
println!("{}", extra::prettier::pretty_string(json_obj).unwrap());
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

- Macros

```rust
let json_obj = json!({
    "name" => "Ammar Dev",
    "Age" => 99
}); // will produce JsonObject

let json_arr = json!([
    1,
    2,
    "String 1",
    "String 2"
]); // will produce JsonArray

let json_val = json!("Hello"); // will Produce JsonValue
```

# Implementing

- Json Entity

```rust
struct User {
    name: String,
    age: i32,
}

impl JsonEntity for User {
    fn to_json_object(&self) -> JsonObject {
        let mut obj = JsonObject::new();
        obj.set("name", &self.name);
        obj.set("age", self.age);
        obj
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let u = User {
        name: "Ammar Dev".to_string(),
        age: 22,
    };


    println!("{}", u.to_json_object());

    Ok(())
}
```

```json
{"name": "Ammar Dev", "age": 22}
```

# Copyrights

All rights reserved for Ammar Dev <br>
This project is licensed with [MIT LICENSE](/LICENSE)




