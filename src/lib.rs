use std::error::Error;
use crate::base::json_o::JsonObject;
use crate::base::json_v::JsonValue;

mod base;
mod extra;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_json = r#"
    {
        "name": "John Doe",
        "age": 30,
        "email": "johndoe@example.com",
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "state": "CA",
            "postalCode": "12345"
        },
        "phoneNumbers": [
            {
                "type": "home",
                "number": "555-555-5555"
            },
            {
                "type": "work",
                "number": "555-555-5556"
            }
        ],
        "isActive": true
    }
    "#;

    let json_value = extra::parser::JsonParser::parse(&raw_json)?;

    let mut json_obj: JsonObject = match json_value {
        JsonValue::Object(obj) => obj,
        _ => return Err("Parsed JSON is not an object".into()),
    };

    let mut json_obj2 = json!({
        "H" => "RR",
        "B" => 223929322232323i64
    });

    let arr = json!(["HBS", 1, 2]);
    json_obj2.set("S", arr);
    json_obj.set("tst", json_obj2);
    json_obj.set("isActive", false);
    json_obj.del("address");
    extra::prettier::set_indent(2);
    println!("{}", extra::prettier::pretty_string(json_obj)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        main().unwrap();
    }
}
