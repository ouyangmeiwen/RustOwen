use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    name: String,
    age: u32,
}
pub fn testjson() {
    // Create an instance of the struct
    let person = MyStruct {
        name: String::from("Alice"),
        age: 30,
    };

    // Serialize the struct to a JSON string
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back to a struct
    let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
