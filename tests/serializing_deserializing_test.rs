#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap};
    use indexmap::IndexMap;
    use rusty_json_serialization::JsonEntity;
    use rusty_json::base::{JsonArray, JsonObject, JsonValue};
    use rusty_json::extra::JsonEntity;

    #[derive(Clone, Debug, PartialEq, JsonEntity)]
    struct TestStruct {
        name: String,
        age: i32,
    }

    #[test]
    fn test_serialize_deserialize_vec() {
        let vec = vec![1, 2, 3];
        let json = vec.to_json();
        assert_eq!(json, JsonValue::Array(JsonArray::from(vec![JsonValue::Number(1.into()), JsonValue::Number(2.into()), JsonValue::Number(3.into())])));

        let raw_json = json.to_string();
        let parsed_vec: Vec<i32> = Vec::from_json(&raw_json).unwrap();
        assert_eq!(vec, parsed_vec);
    }

    #[test]
    fn test_serialize_deserialize_hashmap() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let json = map.to_json();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("key1".to_string(), JsonValue::String("value1".to_string()));
            obj.set("key2".to_string(), JsonValue::String("value2".to_string()));
            obj
        });
        assert_eq!(json, expected_json);

        let raw_json = json.to_string();
        let parsed_map: HashMap<String, String> = HashMap::from_json(&raw_json).unwrap();
        assert_eq!(map, parsed_map);
    }

    #[test]
    fn test_serialize_deserialize_indexmap() {
        let mut map = IndexMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let json = map.to_json();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("key1".to_string(), JsonValue::String("value1".to_string()));
            obj.set("key2".to_string(), JsonValue::String("value2".to_string()));
            obj
        });
        assert_eq!(json, expected_json);

        let raw_json = json.to_string();
        let parsed_map: IndexMap<String, String> = IndexMap::from_json(&raw_json).unwrap();
        assert_eq!(map, parsed_map);
    }

    #[test]
    fn test_serialize_deserialize_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let json = map.to_json();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("key1".to_string(), JsonValue::String("value1".to_string()));
            obj.set("key2".to_string(), JsonValue::String("value2".to_string()));
            obj
        });
        assert_eq!(json, expected_json);

        let raw_json = json.to_string();
        let parsed_map: BTreeMap<String, String> = BTreeMap::from_json(&raw_json).unwrap();
        assert_eq!(map, parsed_map);
    }

    #[test]
    fn test_serialize_deserialize_struct() {
        let test_struct = TestStruct {
            name: "Ammar".to_string(),
            age: 27,
        };
        let json = test_struct.to_json();
        let expected_json = JsonValue::Object({
            let mut obj = JsonObject::new();
            obj.set("name".to_string(), JsonValue::String("Ammar".to_string()));
            obj.set("age".to_string(), JsonValue::Number(27.into()));
            obj
        });
        assert_eq!(json, expected_json);

        let raw_json = json.to_string();
        let parsed_struct: TestStruct = TestStruct::from_json(&raw_json).unwrap();
        assert_eq!(test_struct, parsed_struct);
    }
}
