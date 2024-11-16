use serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date: ", ""))
}

fn main() {
    let event = Event {
        name: "Concert".to_string(),
        date: "2024-11-15".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Serialization error");
    println!("Serialized JSON with castom date: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Deserialization error");
    println!("Deserialized event: {:?}", deserialized_event);
}